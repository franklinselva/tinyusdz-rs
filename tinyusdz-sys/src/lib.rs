//! Raw FFI bindings for tinyusdz.
//!
//! This crate provides unsafe FFI bindings to the tinyusdz C API.
//! For a safe Rust API, use the `tinyusdz-rs` crate instead.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::all)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_lifecycle() {
        unsafe {
            // Create a new stage
            let stage = c_tinyusd_stage_new();
            assert!(!stage.is_null(), "Failed to create stage");

            // Free the stage
            let result = c_tinyusd_stage_free(stage);
            assert_eq!(result, 1, "Failed to free stage");
        }
    }

    #[test]
    fn test_string_operations() {
        unsafe {
            // Create a new string
            let s = c_tinyusd_string_new(b"hello\0".as_ptr() as *const i8);
            assert!(!s.is_null(), "Failed to create string");

            // Get the string content
            let ptr = c_tinyusd_string_str(s);
            assert!(!ptr.is_null(), "Failed to get string content");

            // Get the size
            let size = c_tinyusd_string_size(s);
            assert_eq!(size, 5, "String size mismatch");

            // Free the string
            let result = c_tinyusd_string_free(s);
            assert_eq!(result, 1, "Failed to free string");
        }
    }

    #[test]
    fn test_token_operations() {
        unsafe {
            // Create a new token
            let token = c_tinyusd_token_new(b"test_token\0".as_ptr() as *const i8);
            assert!(!token.is_null(), "Failed to create token");

            // Get the token string
            let ptr = c_tinyusd_token_str(token);
            assert!(!ptr.is_null(), "Failed to get token string");

            // Get the size
            let size = c_tinyusd_token_size(token);
            assert_eq!(size, 10, "Token size mismatch");

            // Free the token
            let result = c_tinyusd_token_free(token);
            assert_eq!(result, 1, "Failed to free token");
        }
    }
}
