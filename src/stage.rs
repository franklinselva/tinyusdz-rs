//! USD Stage operations.

use std::ffi::{CStr, CString};
use std::path::Path;
use std::sync::Mutex;

use crate::error::{Error, Result};
use crate::prim::Prim;

// Wrapper for raw pointer that implements Send (we ensure safety through mutex)
#[derive(Clone, Copy)]
struct PrimPtr(*const tinyusdz_sys::CTinyUSDPrim);
unsafe impl Send for PrimPtr {}

// Global storage for collected prims during traversal (needed because C callback has no userdata)
static TRAVERSAL_PRIMS: Mutex<Vec<PrimPtr>> = Mutex::new(Vec::new());

/// A USD Stage represents the root of a USD scene graph.
///
/// The stage is the main entry point for loading and traversing USD files.
pub struct Stage {
    pub(crate) inner: *mut tinyusdz_sys::CTinyUSDStage,
}

// Safety: Stage owns its inner pointer and manages its lifetime
unsafe impl Send for Stage {}
unsafe impl Sync for Stage {}

impl Stage {
    /// Creates a new empty stage.
    pub fn new() -> Result<Self> {
        let inner = unsafe { tinyusdz_sys::c_tinyusd_stage_new() };
        if inner.is_null() {
            return Err(Error::NullPointer);
        }
        Ok(Stage { inner })
    }

    /// Opens a USD file from the filesystem.
    ///
    /// Automatically detects the format (USDA, USDC, or USDZ).
    ///
    /// # Arguments
    /// * `path` - Path to the USD file
    ///
    /// # Example
    /// ```no_run
    /// use tinyusdz_rs::Stage;
    ///
    /// let stage = Stage::open("model.usdz").unwrap();
    /// for prim in stage.traverse() {
    ///     println!("{}: {}", prim.path(), prim.type_name());
    /// }
    /// ```
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path
            .as_ref()
            .to_str()
            .ok_or_else(|| Error::InvalidPath("Path is not valid UTF-8".into()))?;
        let c_path = CString::new(path_str)?;

        let stage = Self::new()?;

        let warn = unsafe { tinyusdz_sys::c_tinyusd_string_new_empty() };
        let err = unsafe { tinyusdz_sys::c_tinyusd_string_new_empty() };

        let result = unsafe {
            tinyusdz_sys::c_tinyusd_load_usd_from_file(c_path.as_ptr(), stage.inner, warn, err)
        };

        // Get error message if failed
        let error_msg = if result == 0 {
            unsafe {
                let err_ptr = tinyusdz_sys::c_tinyusd_string_str(err);
                if !err_ptr.is_null() {
                    CStr::from_ptr(err_ptr).to_string_lossy().into_owned()
                } else {
                    "Unknown error".to_string()
                }
            }
        } else {
            String::new()
        };

        // Free the warning and error strings
        unsafe {
            if !warn.is_null() {
                tinyusdz_sys::c_tinyusd_string_free(warn);
            }
            if !err.is_null() {
                tinyusdz_sys::c_tinyusd_string_free(err);
            }
        }

        if result == 0 {
            return Err(Error::LoadError(error_msg));
        }

        Ok(stage)
    }

    /// Loads a USD stage from USDA (ASCII) data in memory.
    ///
    /// Note: This writes to a temporary file and loads from there,
    /// as the C API memory loading functions don't populate a stage directly.
    pub fn from_usda(data: &[u8]) -> Result<Self> {
        let temp_file = tempfile_for_data(data, ".usda")?;
        Self::open(temp_file.path())
    }

    /// Loads a USD stage from USDC (binary Crate) data in memory.
    ///
    /// Note: This writes to a temporary file and loads from there,
    /// as the C API memory loading functions don't populate a stage directly.
    pub fn from_usdc(data: &[u8]) -> Result<Self> {
        let temp_file = tempfile_for_data(data, ".usdc")?;
        Self::open(temp_file.path())
    }

    /// Loads a USD stage from USDZ (ZIP archive) data in memory.
    ///
    /// Note: This writes to a temporary file and loads from there,
    /// as the C API memory loading functions don't populate a stage directly.
    pub fn from_usdz(data: &[u8]) -> Result<Self> {
        let temp_file = tempfile_for_data(data, ".usdz")?;
        Self::open(temp_file.path())
    }

    /// Returns an iterator over all prims in the stage using depth-first traversal.
    pub fn traverse(&self) -> StageTraversal<'_> {
        StageTraversal::new(self)
    }

    /// Returns the stage as a USDA string.
    pub fn to_string(&self) -> Result<String> {
        unsafe {
            let s = tinyusdz_sys::c_tinyusd_string_new_empty();
            if s.is_null() {
                return Err(Error::NullPointer);
            }

            let result = tinyusdz_sys::c_tinyusd_stage_to_string(self.inner, s);
            if result == 0 {
                tinyusdz_sys::c_tinyusd_string_free(s);
                return Err(Error::LoadError("Failed to convert stage to string".into()));
            }

            let ptr = tinyusdz_sys::c_tinyusd_string_str(s);
            let result_str = if !ptr.is_null() {
                CStr::from_ptr(ptr).to_string_lossy().into_owned()
            } else {
                String::new()
            };

            tinyusdz_sys::c_tinyusd_string_free(s);
            Ok(result_str)
        }
    }
}

impl Default for Stage {
    fn default() -> Self {
        Self::new().expect("Failed to create default stage")
    }
}

impl Drop for Stage {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                tinyusdz_sys::c_tinyusd_stage_free(self.inner);
            }
        }
    }
}

/// Iterator over prims in a stage.
pub struct StageTraversal<'a> {
    #[allow(dead_code)]
    stage: &'a Stage,
    prims: Vec<Prim<'a>>,
    index: usize,
}

impl<'a> StageTraversal<'a> {
    fn new(stage: &'a Stage) -> Self {
        // Clear previous traversal data
        {
            let mut guard = TRAVERSAL_PRIMS.lock().unwrap();
            guard.clear();
        }

        // Callback that collects prims into global storage
        unsafe extern "C" fn collect_prim(
            prim: *const tinyusdz_sys::CTinyUSDPrim,
            _path: *const tinyusdz_sys::CTinyUSDPath,
        ) -> i32 {
            if prim.is_null() {
                return 1; // continue
            }

            if let Ok(mut guard) = TRAVERSAL_PRIMS.lock() {
                guard.push(PrimPtr(prim));
            }
            1 // continue traversal
        }

        // Perform traversal
        unsafe {
            let err = tinyusdz_sys::c_tinyusd_string_new_empty();

            tinyusdz_sys::c_tinyusd_stage_traverse(stage.inner, Some(collect_prim), err);

            if !err.is_null() {
                tinyusdz_sys::c_tinyusd_string_free(err);
            }
        }

        // Collect results
        let prims: Vec<Prim<'a>> = {
            let guard = TRAVERSAL_PRIMS.lock().unwrap();
            guard.iter().map(|ptr| unsafe { Prim::from_ptr(ptr.0) }).collect()
        };

        StageTraversal {
            stage,
            prims,
            index: 0,
        }
    }
}

impl<'a> Iterator for StageTraversal<'a> {
    type Item = Prim<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.prims.len() {
            let prim = self.prims[self.index].clone();
            self.index += 1;
            Some(prim)
        } else {
            None
        }
    }
}

/// Helper to create a temp file with data
struct TempFile {
    path: std::path::PathBuf,
}

impl TempFile {
    fn path(&self) -> &std::path::Path {
        &self.path
    }
}

fn tempfile_for_data(data: &[u8], extension: &str) -> Result<TempFile> {
    use std::io::Write;

    let temp_dir = std::env::temp_dir();
    let filename = format!("tinyusdz_rs_{}{}", std::process::id(), extension);
    let path = temp_dir.join(filename);

    let mut file = std::fs::File::create(&path)?;
    file.write_all(data)?;
    file.sync_all()?;

    Ok(TempFile { path })
}
