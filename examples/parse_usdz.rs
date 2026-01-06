//! Basic USD parsing example.
//!
//! Usage: cargo run --example parse_usdz -- <path/to/file.usdz>

use std::env;
use tinyusdz_rs::{MeshExtractor, Stage};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path/to/file.usd[z]>", args[0]);
        eprintln!("Example: {} model.usdz", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    println!("Loading: {}", path);

    // Load the stage
    let stage = match Stage::open(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error loading USD file: {}", e);
            std::process::exit(1);
        }
    };

    // Count prims by type
    let mut total_prims = 0;
    let mut mesh_count = 0;
    let mut xform_count = 0;
    let mut material_count = 0;
    let mut other_count = 0;

    for prim in stage.traverse() {
        total_prims += 1;
        match prim.type_name() {
            "Mesh" => mesh_count += 1,
            "Xform" => xform_count += 1,
            "Material" => material_count += 1,
            _ => other_count += 1,
        }
    }

    println!("\nScene Statistics:");
    println!("  Total prims: {}", total_prims);
    println!("  Meshes: {}", mesh_count);
    println!("  Transforms: {}", xform_count);
    println!("  Materials: {}", material_count);
    println!("  Other: {}", other_count);

    // Extract mesh information
    let extractor = MeshExtractor::new(&stage);
    let meshes: Vec<_> = extractor.collect();

    if !meshes.is_empty() {
        println!("\nMesh Details:");
        let mut total_vertices = 0;
        let mut total_faces = 0;

        for mesh in &meshes {
            println!("  {}", mesh.name);
            println!("    Vertices: {}", mesh.vertex_count());
            println!("    Faces: {}", mesh.face_count());
            println!("    Has UVs: {}", mesh.has_uvs());
            println!("    Has normals: {}", mesh.has_normals());

            total_vertices += mesh.vertex_count();
            total_faces += mesh.face_count();
        }

        println!("\nTotals:");
        println!("  Total vertices: {}", total_vertices);
        println!("  Total faces: {}", total_faces);
    }

    println!("\nDone!");
}
