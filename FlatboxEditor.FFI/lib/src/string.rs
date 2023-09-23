use std::ffi::{c_char, CString};

/// Create raw C-string
pub fn create_cstring(string: &str) -> *mut c_char {
    let s = CString::new(string).unwrap();
    s.into_raw()
}

/// FFI function to free C-string memory from external library
#[no_mangle]
pub extern "C" fn free_cstring(ptr: *mut c_char) {
    unsafe { CString::from_raw(ptr) };
}