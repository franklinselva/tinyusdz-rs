//! USD to GLB converter example.
//!
//! Converts a USD file to GLB format using the gltf crate.
//!
//! Usage: cargo run --example usd_to_glb -- <input.usdz> <output.glb>

use std::env;
use std::fs::File;
use std::io::Write;
use tinyusdz_rs::{MeshExtractor, Stage};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <input.usd[z]> <output.glb>", args[0]);
        eprintln!("Example: {} model.usdz model.glb", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    println!("Loading: {}", input_path);

    // Load the USD stage
    let stage = match Stage::open(input_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error loading USD file: {}", e);
            std::process::exit(1);
        }
    };

    // Extract meshes
    let extractor = MeshExtractor::new(&stage);
    let meshes: Vec<_> = extractor.collect();

    if meshes.is_empty() {
        eprintln!("No meshes found in USD file");
        std::process::exit(1);
    }

    println!("Found {} meshes", meshes.len());

    // Build glTF JSON structure
    let mut gltf = build_gltf(&meshes);

    // Write to file
    match write_glb(output_path, &mut gltf, &meshes) {
        Ok(_) => println!("Wrote: {}", output_path),
        Err(e) => {
            eprintln!("Error writing GLB: {}", e);
            std::process::exit(1);
        }
    }

    println!("Done!");
}

fn build_gltf(meshes: &[tinyusdz_rs::Mesh]) -> gltf_json::Root {
    use gltf_json as json;

    let mut root = json::Root::default();

    // Create a buffer to hold all binary data
    let mut buffer_data: Vec<u8> = Vec::new();

    // Create buffer views and accessors for each mesh
    let mut mesh_primitives = Vec::new();

    for (_mesh_idx, mesh) in meshes.iter().enumerate() {
        // Triangulate the mesh
        let triangulated = mesh.triangulate();

        // Skip empty meshes
        if triangulated.points.is_empty() || triangulated.face_vertex_indices.is_empty() {
            continue;
        }

        let vertex_start = buffer_data.len();

        // Write vertex positions
        for point in &triangulated.points {
            for &coord in point {
                buffer_data.extend_from_slice(&coord.to_le_bytes());
            }
        }

        let vertex_end = buffer_data.len();
        let vertex_byte_length = vertex_end - vertex_start;

        // Pad to 4-byte boundary
        while buffer_data.len() % 4 != 0 {
            buffer_data.push(0);
        }

        let index_start = buffer_data.len();

        // Write indices
        for &idx in &triangulated.face_vertex_indices {
            buffer_data.extend_from_slice(&(idx as u32).to_le_bytes());
        }

        let index_end = buffer_data.len();
        let index_byte_length = index_end - index_start;

        // Pad to 4-byte boundary
        while buffer_data.len() % 4 != 0 {
            buffer_data.push(0);
        }

        // Calculate bounding box
        let mut min = [f32::MAX; 3];
        let mut max = [f32::MIN; 3];
        for point in &triangulated.points {
            for i in 0..3 {
                min[i] = min[i].min(point[i]);
                max[i] = max[i].max(point[i]);
            }
        }

        // Create buffer views
        let position_buffer_view_idx = root.buffer_views.len() as u32;
        root.buffer_views.push(json::buffer::View {
            buffer: json::Index::new(0),
            byte_length: json::validation::USize64(vertex_byte_length as u64),
            byte_offset: Some(json::validation::USize64(vertex_start as u64)),
            byte_stride: Some(json::buffer::Stride(12)), // 3 * f32
            target: Some(json::validation::Checked::Valid(
                json::buffer::Target::ArrayBuffer,
            )),
            name: None,
            extensions: None,
            extras: Default::default(),
        });

        let index_buffer_view_idx = root.buffer_views.len() as u32;
        root.buffer_views.push(json::buffer::View {
            buffer: json::Index::new(0),
            byte_length: json::validation::USize64(index_byte_length as u64),
            byte_offset: Some(json::validation::USize64(index_start as u64)),
            byte_stride: None,
            target: Some(json::validation::Checked::Valid(
                json::buffer::Target::ElementArrayBuffer,
            )),
            name: None,
            extensions: None,
            extras: Default::default(),
        });

        // Create accessors
        let position_accessor_idx = root.accessors.len() as u32;
        root.accessors.push(json::Accessor {
            buffer_view: Some(json::Index::new(position_buffer_view_idx)),
            byte_offset: Some(json::validation::USize64(0)),
            count: json::validation::USize64(triangulated.points.len() as u64),
            component_type: json::validation::Checked::Valid(json::accessor::GenericComponentType(
                json::accessor::ComponentType::F32,
            )),
            type_: json::validation::Checked::Valid(json::accessor::Type::Vec3),
            min: Some(json::Value::from(min.to_vec())),
            max: Some(json::Value::from(max.to_vec())),
            normalized: false,
            sparse: None,
            name: None,
            extensions: None,
            extras: Default::default(),
        });

        let index_accessor_idx = root.accessors.len() as u32;
        root.accessors.push(json::Accessor {
            buffer_view: Some(json::Index::new(index_buffer_view_idx)),
            byte_offset: Some(json::validation::USize64(0)),
            count: json::validation::USize64(triangulated.face_vertex_indices.len() as u64),
            component_type: json::validation::Checked::Valid(json::accessor::GenericComponentType(
                json::accessor::ComponentType::U32,
            )),
            type_: json::validation::Checked::Valid(json::accessor::Type::Scalar),
            min: None,
            max: None,
            normalized: false,
            sparse: None,
            name: None,
            extensions: None,
            extras: Default::default(),
        });

        // Create primitive
        let mut attributes = std::collections::BTreeMap::new();
        attributes.insert(
            json::validation::Checked::Valid(json::mesh::Semantic::Positions),
            json::Index::new(position_accessor_idx),
        );

        mesh_primitives.push((
            mesh.name.clone(),
            json::mesh::Primitive {
                attributes,
                indices: Some(json::Index::new(index_accessor_idx)),
                material: None,
                mode: json::validation::Checked::Valid(json::mesh::Mode::Triangles),
                targets: None,
                extensions: None,
                extras: Default::default(),
            },
        ));
    }

    // Create meshes
    for (name, primitive) in mesh_primitives {
        let mesh_idx = root.meshes.len() as u32;
        root.meshes.push(json::Mesh {
            name: Some(name.clone()),
            primitives: vec![primitive],
            weights: None,
            extensions: None,
            extras: Default::default(),
        });

        // Create a node for each mesh
        root.nodes.push(json::Node {
            camera: None,
            children: None,
            skin: None,
            matrix: None,
            mesh: Some(json::Index::new(mesh_idx)),
            rotation: None,
            scale: None,
            translation: None,
            weights: None,
            name: Some(name),
            extensions: None,
            extras: Default::default(),
        });
    }

    // Create scene
    let node_indices: Vec<_> = (0..root.nodes.len() as u32).map(json::Index::new).collect();

    root.scenes.push(json::Scene {
        name: Some("Scene".to_string()),
        nodes: node_indices,
        extensions: None,
        extras: Default::default(),
    });

    root.scene = Some(json::Index::new(0));

    // Create buffer
    root.buffers.push(json::Buffer {
        byte_length: json::validation::USize64(buffer_data.len() as u64),
        uri: None, // Embedded in GLB
        name: None,
        extensions: None,
        extras: Default::default(),
    });

    // Store buffer data for later
    // We'll need to pass this separately to write_glb

    root
}

fn write_glb(
    path: &str,
    root: &mut gltf_json::Root,
    meshes: &[tinyusdz_rs::Mesh],
) -> std::io::Result<()> {
    // Rebuild buffer data (same as in build_gltf)
    let mut buffer_data: Vec<u8> = Vec::new();

    for mesh in meshes {
        let triangulated = mesh.triangulate();
        if triangulated.points.is_empty() || triangulated.face_vertex_indices.is_empty() {
            continue;
        }

        // Write vertex positions
        for point in &triangulated.points {
            for &coord in point {
                buffer_data.extend_from_slice(&coord.to_le_bytes());
            }
        }

        // Pad to 4-byte boundary
        while buffer_data.len() % 4 != 0 {
            buffer_data.push(0);
        }

        // Write indices
        for &idx in &triangulated.face_vertex_indices {
            buffer_data.extend_from_slice(&(idx as u32).to_le_bytes());
        }

        // Pad to 4-byte boundary
        while buffer_data.len() % 4 != 0 {
            buffer_data.push(0);
        }
    }

    // Serialize JSON
    let json_string = gltf_json::serialize::to_string(root)?;
    let json_bytes = json_string.as_bytes();

    // Pad JSON to 4-byte boundary
    let json_padding = (4 - (json_bytes.len() % 4)) % 4;
    let json_chunk_length = json_bytes.len() + json_padding;

    // Pad binary data to 4-byte boundary
    let bin_padding = (4 - (buffer_data.len() % 4)) % 4;
    let bin_chunk_length = buffer_data.len() + bin_padding;

    // Calculate total length
    let total_length = 12 + // GLB header
        8 + json_chunk_length + // JSON chunk
        8 + bin_chunk_length; // BIN chunk

    // Write GLB file
    let mut file = File::create(path)?;

    // GLB header
    file.write_all(b"glTF")?; // magic
    file.write_all(&2u32.to_le_bytes())?; // version
    file.write_all(&(total_length as u32).to_le_bytes())?; // total length

    // JSON chunk
    file.write_all(&(json_chunk_length as u32).to_le_bytes())?; // chunk length
    file.write_all(&0x4E4F534Au32.to_le_bytes())?; // chunk type "JSON"
    file.write_all(json_bytes)?;
    for _ in 0..json_padding {
        file.write_all(&[0x20])?; // space padding for JSON
    }

    // BIN chunk
    file.write_all(&(bin_chunk_length as u32).to_le_bytes())?; // chunk length
    file.write_all(&0x004E4942u32.to_le_bytes())?; // chunk type "BIN\0"
    file.write_all(&buffer_data)?;
    for _ in 0..bin_padding {
        file.write_all(&[0])?; // zero padding for binary
    }

    Ok(())
}
