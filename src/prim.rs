//! USD Prim (primitive) handling.

use std::ffi::CStr;
use std::marker::PhantomData;

/// A USD Prim represents a node in the scene hierarchy.
///
/// Prims can have children, attributes, and metadata.
#[derive(Clone)]
pub struct Prim<'a> {
    pub(crate) inner: *const tinyusdz_sys::CTinyUSDPrim,
    pub(crate) _marker: PhantomData<&'a ()>,
}

// Safety: Prim borrows from Stage which manages lifetime
unsafe impl<'a> Send for Prim<'a> {}
unsafe impl<'a> Sync for Prim<'a> {}

impl<'a> Prim<'a> {
    /// Creates a Prim from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid for the lifetime 'a.
    pub(crate) unsafe fn from_ptr(ptr: *const tinyusdz_sys::CTinyUSDPrim) -> Self {
        Prim {
            inner: ptr,
            _marker: PhantomData,
        }
    }

    /// Returns the type name of this prim (e.g., "Mesh", "Xform", "Material").
    pub fn type_name(&self) -> &str {
        unsafe {
            let ptr = tinyusdz_sys::c_tinyusd_prim_type(self.inner);
            if ptr.is_null() {
                return "";
            }
            CStr::from_ptr(ptr).to_str().unwrap_or("")
        }
    }

    /// Returns the element name (local name) of this prim.
    pub fn name(&self) -> &str {
        unsafe {
            let ptr = tinyusdz_sys::c_tinyusd_prim_element_name(self.inner);
            if ptr.is_null() {
                return "";
            }
            CStr::from_ptr(ptr).to_str().unwrap_or("")
        }
    }

    /// Returns the full path of this prim in the scene hierarchy.
    pub fn path(&self) -> String {
        // The path is typically constructed from the element names
        // For now, we'll return the element name as a simple path
        format!("/{}", self.name())
    }

    /// Returns the number of child prims.
    pub fn num_children(&self) -> usize {
        unsafe { tinyusdz_sys::c_tinyusd_prim_num_children(self.inner) as usize }
    }

    /// Returns a child prim by index.
    pub fn child(&self, index: usize) -> Option<Prim<'a>> {
        let num = self.num_children();
        if index >= num {
            return None;
        }

        unsafe {
            let mut child_ptr: *const tinyusdz_sys::CTinyUSDPrim = std::ptr::null();
            let result =
                tinyusdz_sys::c_tinyusd_prim_get_child(self.inner, index as u64, &mut child_ptr);

            if result != 0 && !child_ptr.is_null() {
                Some(Prim::from_ptr(child_ptr))
            } else {
                None
            }
        }
    }

    /// Returns an iterator over the children of this prim.
    pub fn children(&self) -> ChildIterator<'a> {
        ChildIterator {
            prim: self.clone(),
            index: 0,
            count: self.num_children(),
        }
    }

    /// Returns the names of all properties on this prim.
    pub fn property_names(&self) -> Vec<String> {
        unsafe {
            let tokens = tinyusdz_sys::c_tinyusd_token_vector_new_empty();
            if tokens.is_null() {
                return Vec::new();
            }

            let result = tinyusdz_sys::c_tinyusd_prim_get_property_names(self.inner, tokens);
            if result == 0 {
                tinyusdz_sys::c_tinyusd_token_vector_free(tokens);
                return Vec::new();
            }

            let count = tinyusdz_sys::c_tinyusd_token_vector_size(tokens);
            let mut names = Vec::with_capacity(count);

            for i in 0..count {
                let ptr = tinyusdz_sys::c_tinyusd_token_vector_str(tokens, i);
                if !ptr.is_null() {
                    if let Ok(s) = CStr::from_ptr(ptr).to_str() {
                        names.push(s.to_string());
                    }
                }
            }

            tinyusdz_sys::c_tinyusd_token_vector_free(tokens);
            names
        }
    }

    /// Returns true if this prim is a Mesh.
    pub fn is_mesh(&self) -> bool {
        self.type_name() == "Mesh"
    }

    /// Returns true if this prim is an Xform (transform).
    pub fn is_xform(&self) -> bool {
        self.type_name() == "Xform"
    }

    /// Returns true if this prim is a Material.
    pub fn is_material(&self) -> bool {
        self.type_name() == "Material"
    }

    /// Returns true if this prim is a Shader.
    pub fn is_shader(&self) -> bool {
        self.type_name() == "Shader"
    }

    /// Returns true if this prim is a Camera.
    pub fn is_camera(&self) -> bool {
        self.type_name() == "Camera"
    }

    /// Returns true if this prim is a light (any light type).
    pub fn is_light(&self) -> bool {
        let type_name = self.type_name();
        type_name.ends_with("Light")
    }

    /// Converts this prim to a debug string representation.
    pub fn to_debug_string(&self) -> String {
        unsafe {
            let s = tinyusdz_sys::c_tinyusd_string_new_empty();
            if s.is_null() {
                return String::new();
            }

            let result = tinyusdz_sys::c_tinyusd_prim_to_string(self.inner, s);
            if result == 0 {
                tinyusdz_sys::c_tinyusd_string_free(s);
                return String::new();
            }

            let ptr = tinyusdz_sys::c_tinyusd_string_str(s);
            let result_str = if !ptr.is_null() {
                CStr::from_ptr(ptr).to_string_lossy().into_owned()
            } else {
                String::new()
            };

            tinyusdz_sys::c_tinyusd_string_free(s);
            result_str
        }
    }
}

impl<'a> std::fmt::Debug for Prim<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Prim")
            .field("name", &self.name())
            .field("type", &self.type_name())
            .field("num_children", &self.num_children())
            .finish()
    }
}

/// Iterator over the children of a prim.
pub struct ChildIterator<'a> {
    prim: Prim<'a>,
    index: usize,
    count: usize,
}

impl<'a> Iterator for ChildIterator<'a> {
    type Item = Prim<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.count {
            let child = self.prim.child(self.index);
            self.index += 1;
            child
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.count - self.index;
        (remaining, Some(remaining))
    }
}

impl<'a> ExactSizeIterator for ChildIterator<'a> {}
