pub mod editor;
pub mod logger;


use std::ffi::{c_char, c_void, CStr};

pub use logger::*;
pub use editor::*;

pub type GlInitFunctionFFI = extern fn(*const c_char) -> *const c_void;

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