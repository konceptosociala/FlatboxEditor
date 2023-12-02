pub mod camera;
pub mod logger;
pub mod model;
pub mod render;
pub mod scene;

pub mod opengl;

use std::ffi::{CStr, c_char};

pub use camera::*;
pub use logger::*;
pub use model::*;
pub use render::*;
pub use scene::*;

///
/// # Safety
/// `ptr` must be a valid const pointer of given type
pub unsafe fn assert_ptr<T>(ptr: *const T) -> &'static T {
    assert!(!ptr.is_null());
    &*ptr
}

///
/// # Safety
/// `ptr` must be a valid mutable pointer of given type
pub unsafe fn assert_ptr_mut<T>(ptr: *mut T) -> &'static mut T {
    assert!(!ptr.is_null());
    &mut *ptr
}

///
/// # Safety
/// `ptr` must be a valid mutable pointer of given type
pub unsafe fn free_ptr<T>(ptr: *mut T) {
    if ptr.is_null() {
        return;
    }
    
    let _ = Box::from_raw(ptr);
}

///
/// # Safety
/// `ptr` must be a valid C string pointer
pub unsafe fn ptr_to_string(ptr: *const c_char) -> &'static str {
    let cstring = CStr::from_ptr(ptr);
    cstring.to_str().unwrap()
}