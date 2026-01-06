//! # tinyusdz-rs
//!
//! Safe Rust bindings for [tinyusdz](https://github.com/lighttransport/tinyusdz),
//! a lightweight USD/USDZ parser and writer.
//!
//! ## Features
//!
//! - Parse USD files (USDA, USDC, USDZ)
//! - Access scene hierarchy (prims, attributes, relationships)
//! - Extract mesh geometry, materials, and transforms
//! - No Python or OpenUSD dependency
//!
//! ## Quick Start
//!
//! ```no_run
//! use tinyusdz_rs::{Stage, MeshExtractor, Result};
//!
//! fn main() -> Result<()> {
//!     // Load a USD file
//!     let stage = Stage::open("model.usdz")?;
//!
//!     // Traverse all prims
//!     for prim in stage.traverse() {
//!         println!("{}: {}", prim.name(), prim.type_name());
//!     }
//!
//!     // Extract meshes
//!     let extractor = MeshExtractor::new(&stage);
//!     for mesh in extractor.meshes() {
//!         println!("Mesh: {} ({} vertices)", mesh.name, mesh.vertex_count());
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Loading USD Files
//!
//! The [`Stage`] type is the main entry point for loading USD files:
//!
//! ```no_run
//! use tinyusdz_rs::Stage;
//!
//! // Load from file (auto-detects format)
//! let stage = Stage::open("model.usdz").unwrap();
//!
//! // Load from memory
//! let data = std::fs::read("model.usdc").unwrap();
//! let stage = Stage::from_usdc(&data).unwrap();
//! ```
//!
//! ## Traversing the Scene
//!
//! Use the [`Stage::traverse`] method to iterate over all prims:
//!
//! ```no_run
//! use tinyusdz_rs::Stage;
//!
//! let stage = Stage::open("scene.usdz").unwrap();
//!
//! for prim in stage.traverse() {
//!     println!("Prim: {} (type: {})", prim.name(), prim.type_name());
//!
//!     // Access attributes
//!     for prop_name in prim.property_names() {
//!         println!("  Property: {}", prop_name);
//!     }
//! }
//! ```
//!
//! ## Extracting Meshes
//!
//! Use [`MeshExtractor`] to extract mesh data:
//!
//! ```no_run
//! use tinyusdz_rs::{Stage, MeshExtractor};
//!
//! let stage = Stage::open("model.usdz").unwrap();
//! let extractor = MeshExtractor::new(&stage);
//!
//! for mesh in extractor.meshes() {
//!     println!("Mesh: {}", mesh.name);
//!     println!("  Vertices: {}", mesh.points.len());
//!     println!("  Faces: {}", mesh.face_vertex_counts.len());
//!     println!("  Has UVs: {}", mesh.has_uvs());
//!     println!("  Has normals: {}", mesh.has_normals());
//! }
//! ```

pub mod attribute;
pub mod error;
pub mod material;
pub mod mesh;
pub mod prim;
pub mod stage;
pub mod value;

// Re-exports
pub use attribute::Attribute;
pub use error::{Error, Result};
pub use material::{Material, MaterialExtractor};
pub use mesh::{Mesh, MeshExtractor};
pub use prim::Prim;
pub use stage::Stage;
pub use value::{Value, ValueType};

/// Detects the format of a USD file by its path.
///
/// Returns the detected format or `None` if unknown.
pub fn detect_format(path: &str) -> Option<Format> {
    use std::ffi::CString;

    let c_path = CString::new(path).ok()?;
    let format = unsafe { tinyusdz_sys::c_tinyusd_detect_format(c_path.as_ptr()) };

    match format {
        tinyusdz_sys::CTinyUSDFormat::C_TINYUSD_FORMAT_USDA => Some(Format::Usda),
        tinyusdz_sys::CTinyUSDFormat::C_TINYUSD_FORMAT_USDC => Some(Format::Usdc),
        tinyusdz_sys::CTinyUSDFormat::C_TINYUSD_FORMAT_USDZ => Some(Format::Usdz),
        _ => None,
    }
}

/// USD file format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Format {
    /// ASCII USD format (.usda)
    Usda,
    /// Binary Crate format (.usdc)
    Usdc,
    /// ZIP archive format (.usdz)
    Usdz,
}

/// Checks if a file is a valid USD file.
pub fn is_usd_file(path: &str) -> bool {
    use std::ffi::CString;

    let c_path = match CString::new(path) {
        Ok(p) => p,
        Err(_) => return false,
    };

    unsafe { tinyusdz_sys::c_tinyusd_is_usd_file(c_path.as_ptr()) != 0 }
}

/// Checks if data in memory is valid USD data.
pub fn is_usd_memory(data: &[u8]) -> bool {
    unsafe { tinyusdz_sys::c_tinyusd_is_usd_memory(data.as_ptr(), data.len()) != 0 }
}
