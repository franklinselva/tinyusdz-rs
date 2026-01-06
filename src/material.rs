//! USD Material handling (UsdPreviewSurface).
//!
//! Note: Due to limitations in the current tinyusdz C API, material property
//! extraction is not yet fully supported. The property/attribute value
//! extraction functions are not yet implemented.

use crate::prim::Prim;
use crate::stage::Stage;

/// A material extracted from USD (UsdPreviewSurface).
///
/// Note: Due to C API limitations, material property extraction is not yet
/// fully functional. Materials will contain default values.
#[derive(Debug, Clone)]
pub struct Material {
    /// The name of the material.
    pub name: String,
    /// Diffuse/albedo color (RGB).
    pub diffuse_color: Option<[f32; 3]>,
    /// Emissive color (RGB).
    pub emissive_color: Option<[f32; 3]>,
    /// Metallic value (0.0 = dielectric, 1.0 = metallic).
    pub metallic: Option<f32>,
    /// Roughness value (0.0 = smooth, 1.0 = rough).
    pub roughness: Option<f32>,
    /// Opacity value (0.0 = transparent, 1.0 = opaque).
    pub opacity: Option<f32>,
    /// Index of refraction.
    pub ior: Option<f32>,
    /// Clearcoat weight.
    pub clearcoat: Option<f32>,
    /// Clearcoat roughness.
    pub clearcoat_roughness: Option<f32>,
    /// Diffuse texture path (if any).
    pub diffuse_texture: Option<String>,
    /// Normal map texture path (if any).
    pub normal_texture: Option<String>,
    /// Metallic/roughness texture path (if any).
    pub metallic_roughness_texture: Option<String>,
    /// Occlusion texture path (if any).
    pub occlusion_texture: Option<String>,
    /// Emissive texture path (if any).
    pub emissive_texture: Option<String>,
}

impl Material {
    /// Creates a new material with default values.
    pub fn new(name: impl Into<String>) -> Self {
        Material {
            name: name.into(),
            diffuse_color: Some([0.8, 0.8, 0.8]), // Default gray
            emissive_color: None,
            metallic: Some(0.0),
            roughness: Some(0.5),
            opacity: Some(1.0),
            ior: Some(1.5),
            clearcoat: None,
            clearcoat_roughness: None,
            diffuse_texture: None,
            normal_texture: None,
            metallic_roughness_texture: None,
            occlusion_texture: None,
            emissive_texture: None,
        }
    }

    /// Returns true if this material has any textures.
    pub fn has_textures(&self) -> bool {
        self.diffuse_texture.is_some()
            || self.normal_texture.is_some()
            || self.metallic_roughness_texture.is_some()
            || self.occlusion_texture.is_some()
            || self.emissive_texture.is_some()
    }

    /// Returns true if this material is transparent.
    pub fn is_transparent(&self) -> bool {
        self.opacity.map(|o| o < 1.0).unwrap_or(false)
    }

    /// Returns true if this material is metallic.
    pub fn is_metallic(&self) -> bool {
        self.metallic.map(|m| m > 0.5).unwrap_or(false)
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new("default")
    }
}

/// Utility to extract materials from a USD stage.
///
/// Note: Due to limitations in the current tinyusdz C API, material property
/// extraction is not yet fully supported. Materials will be identified but
/// may contain only default values.
pub struct MaterialExtractor<'a> {
    stage: &'a Stage,
}

impl<'a> MaterialExtractor<'a> {
    /// Creates a new material extractor for the given stage.
    pub fn new(stage: &'a Stage) -> Self {
        MaterialExtractor { stage }
    }

    /// Returns an iterator over all materials in the stage.
    ///
    /// Note: Due to C API limitations, materials will have names but
    /// may not contain actual material property values.
    pub fn materials(&self) -> impl Iterator<Item = Material> + '_ {
        self.stage.traverse().filter_map(|prim| {
            if prim.is_material() {
                Some(Self::extract_material(&prim))
            } else {
                None
            }
        })
    }

    /// Extracts material data from a Material prim.
    fn extract_material(prim: &Prim<'_>) -> Material {
        let mat = Material::new(prim.name());

        // Note: Due to C API limitations, we can only get property names
        // but not their values. The property extraction functions
        // are not yet implemented in tinyusdz.
        let _property_names = prim.property_names();

        // Future: When C API supports it, extract shader parameters:
        // - inputs:diffuseColor
        // - inputs:metallic
        // - inputs:roughness
        // - inputs:opacity
        // - etc.

        mat
    }

    /// Extracts all materials and returns them as a vector.
    pub fn collect(&self) -> Vec<Material> {
        self.materials().collect()
    }
}
