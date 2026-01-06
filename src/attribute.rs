//! USD Attribute handling.
//!
//! Note: Attribute extraction is limited in the current C API version.
//! Property/attribute access functions are not yet fully implemented in tinyusdz.

use crate::value::Value;

/// A USD Attribute holds a value on a prim.
///
/// Note: Due to limitations in the current C API, attribute value extraction
/// is not yet fully supported. This struct serves as a placeholder for
/// future functionality.
#[derive(Debug)]
pub struct Attribute {
    // Placeholder - the C API doesn't yet support property extraction
    _private: (),
}

impl Attribute {
    /// Returns the value of this attribute.
    ///
    /// Note: Currently returns Value::None due to C API limitations.
    pub fn value(&self) -> Value {
        Value::None
    }

    /// Returns true if this attribute is an array type.
    pub fn is_array(&self) -> bool {
        false
    }
}
