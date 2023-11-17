use std::ffi::{c_char, CStr};

use flatbox_assets::scene::Scene;
use flatbox_assets::ron;

#[no_mangle]
pub extern "C" fn scene_new() -> *mut Scene {
    Box::into_raw(Box::default())
}

///
/// # Safety
/// `path` must be a valid C string pointer
#[no_mangle]
pub unsafe extern "C" fn scene_open(path: *const c_char) -> *mut Scene {
    let cstring = CStr::from_ptr(path);
    let path = cstring.to_str().unwrap();
    match std::fs::read_to_string(path) {
        Ok(ser) => match ron::from_str(&ser) {
            Ok(scene) => Box::into_raw(Box::new(scene)),
            Err(e) => {
                eprintln!("Cannot load scene `{}`:\n{}", path, e);
                scene_new()
            },
        },
        Err(e) => {
            eprintln!("Cannot open file `{}`: {}", path, e);
            scene_new()
        },
    }
}

///
/// # Safety
/// `ptr` must be a valid `Scene` pointer
/// `path` must be a valid C string pointer
#[no_mangle]
pub unsafe extern "C" fn scene_save(ptr: *const Scene, path: *const c_char) {
    let cstring = CStr::from_ptr(path);
    let path = cstring.to_str().unwrap();

    let scene = {
        assert!(!ptr.is_null());
        &*ptr
    };

    let scene = match ron::ser::to_string_pretty(&scene, ron::ser::PrettyConfig::default()) {
        Ok(scene) => scene,
        Err(e) => {
            eprintln!("Cannot save scene: {}", e);
            return;
        }
    };

    if let Err(e) = std::fs::write(path, scene) {
        eprintln!("Cannot save scene: {}", e);
    }
}

///
/// # Safety
/// `ptr` must be a valid `Scene` pointer
#[no_mangle]
pub unsafe extern "C" fn scene_free(ptr: *mut Scene) {
    if ptr.is_null() {
        return;
    }
    
    let _ = Box::from_raw(ptr);
}