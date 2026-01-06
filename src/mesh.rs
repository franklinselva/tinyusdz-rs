//! Mesh extraction from USD prims.
//!
//! Note: Due to limitations in the current tinyusdz C API, mesh geometry data
//! (vertices, indices, UVs, normals) cannot be fully extracted yet.
//! The property/attribute value extraction functions are not yet implemented.
//!
//! This module provides the infrastructure for mesh extraction which will work
//! once the C API is more complete.

use crate::prim::Prim;
use crate::stage::Stage;

/// A mesh extracted from a USD Mesh prim.
///
/// Provides access to vertex positions, face data, normals, and UVs.
///
/// Note: Due to C API limitations, geometry data extraction is not yet
/// fully functional. The mesh will contain metadata but not actual geometry data.
#[derive(Debug, Clone)]
pub struct Mesh {
    /// The name of the mesh.
    pub name: String,
    /// Vertex positions (x, y, z).
    pub points: Vec<[f32; 3]>,
    /// Number of vertices per face.
    pub face_vertex_counts: Vec<i32>,
    /// Indices into the points array for each face vertex.
    pub face_vertex_indices: Vec<i32>,
    /// Vertex normals (optional).
    pub normals: Option<Vec<[f32; 3]>>,
    /// Texture coordinates (optional).
    pub uvs: Option<Vec<[f32; 2]>>,
    /// Local transformation matrix (4x4).
    pub local_transform: [[f64; 4]; 4],
    /// World transformation matrix (4x4).
    pub world_transform: [[f64; 4]; 4],
}

impl Mesh {
    /// Creates a new empty mesh.
    pub fn new(name: impl Into<String>) -> Self {
        Mesh {
            name: name.into(),
            points: Vec::new(),
            face_vertex_counts: Vec::new(),
            face_vertex_indices: Vec::new(),
            normals: None,
            uvs: None,
            local_transform: identity_matrix(),
            world_transform: identity_matrix(),
        }
    }

    /// Returns the number of vertices in this mesh.
    pub fn vertex_count(&self) -> usize {
        self.points.len()
    }

    /// Returns the number of faces in this mesh.
    pub fn face_count(&self) -> usize {
        self.face_vertex_counts.len()
    }

    /// Returns true if this mesh has normals.
    pub fn has_normals(&self) -> bool {
        self.normals.is_some()
    }

    /// Returns true if this mesh has UVs.
    pub fn has_uvs(&self) -> bool {
        self.uvs.is_some()
    }

    /// Triangulates the mesh if it contains non-triangle faces.
    ///
    /// Returns a new mesh with only triangular faces.
    pub fn triangulate(&self) -> Mesh {
        let mut result = Mesh::new(&self.name);
        result.points = self.points.clone();
        result.local_transform = self.local_transform;
        result.world_transform = self.world_transform;

        if self.face_vertex_indices.is_empty() {
            return result;
        }

        let mut new_indices = Vec::new();
        let mut index_offset = 0;

        for &count in &self.face_vertex_counts {
            if count < 3 {
                index_offset += count as usize;
                continue;
            }

            // Fan triangulation
            let first = self.face_vertex_indices[index_offset];
            for i in 1..(count as usize - 1) {
                new_indices.push(first);
                new_indices.push(self.face_vertex_indices[index_offset + i]);
                new_indices.push(self.face_vertex_indices[index_offset + i + 1]);
            }

            index_offset += count as usize;
        }

        result.face_vertex_counts = vec![3; new_indices.len() / 3];
        result.face_vertex_indices = new_indices;

        result
    }
}

/// Utility to extract meshes from a USD stage.
///
/// Note: Due to limitations in the current tinyusdz C API, actual geometry
/// data cannot be extracted. This extractor identifies Mesh prims and creates
/// placeholder Mesh objects with names and property lists.
pub struct MeshExtractor<'a> {
    stage: &'a Stage,
}

impl<'a> MeshExtractor<'a> {
    /// Creates a new mesh extractor for the given stage.
    pub fn new(stage: &'a Stage) -> Self {
        MeshExtractor { stage }
    }

    /// Returns an iterator over all meshes in the stage.
    ///
    /// Note: The returned meshes will have names and property information
    /// but may not contain actual geometry data due to C API limitations.
    pub fn meshes(&self) -> impl Iterator<Item = Mesh> + '_ {
        self.stage.traverse().filter_map(|prim| {
            if prim.is_mesh() {
                Some(Self::extract_mesh(&prim))
            } else {
                None
            }
        })
    }

    /// Extracts mesh data from a prim.
    fn extract_mesh(prim: &Prim<'_>) -> Mesh {
        let mesh = Mesh::new(prim.name());

        // Note: Due to C API limitations, we can only get property names
        // but not their values. The property extraction functions
        // (c_tinyusd_prim_property_get, etc.) are not yet implemented.
        let _property_names = prim.property_names();

        // Future: When C API supports it, extract:
        // - points (vertex positions)
        // - faceVertexCounts
        // - faceVertexIndices
        // - normals
        // - primvars:st (UVs)
        // - xformOp:transform (transform matrix)

        mesh
    }

    /// Extracts all meshes and returns them as a vector.
    pub fn collect(&self) -> Vec<Mesh> {
        self.meshes().collect()
    }
}

/// Returns a 4x4 identity matrix.
fn identity_matrix() -> [[f64; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

/// Multiplies two 4x4 matrices.
#[allow(clippy::needless_range_loop)]
pub fn matrix_multiply(a: [[f64; 4]; 4], b: [[f64; 4]; 4]) -> [[f64; 4]; 4] {
    let mut result = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}

/// Converts a 4x4 f64 matrix to f32.
pub fn matrix_to_f32(m: [[f64; 4]; 4]) -> [[f32; 4]; 4] {
    [
        [
            m[0][0] as f32,
            m[0][1] as f32,
            m[0][2] as f32,
            m[0][3] as f32,
        ],
        [
            m[1][0] as f32,
            m[1][1] as f32,
            m[1][2] as f32,
            m[1][3] as f32,
        ],
        [
            m[2][0] as f32,
            m[2][1] as f32,
            m[2][2] as f32,
            m[2][3] as f32,
        ],
        [
            m[3][0] as f32,
            m[3][1] as f32,
            m[3][2] as f32,
            m[3][3] as f32,
        ],
    ]
}
