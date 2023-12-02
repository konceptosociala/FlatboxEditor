use std::ffi::c_char;
use flatbox_core::{logger::{error, debug}, math::transform::Transform};
use flatbox_assets::{ron, scene::{Scene, SerializableEntity}, entity};
use flatbox_render::pbr::{material::DefaultMaterial, model::Model, texture::Texture};

use crate::{assert_ptr_mut, assert_ptr, ptr_to_string, free_ptr};

#[no_mangle]
pub extern "C" fn scene_new() -> *mut Scene {
    debug!("Scene::new()");
    Box::into_raw(Box::default())
}

///
/// # Safety
/// `path` must be a valid C string pointer
#[no_mangle]
pub unsafe extern "C" fn scene_open(path: *const c_char) -> *mut Scene {
    let path = ptr_to_string(path);
    debug!("Scene::open()");
    match std::fs::read_to_string(path) {
        Ok(ser) => match ron::from_str(&ser) {
            Ok(scene) => Box::into_raw(Box::new(scene)),
            Err(e) => {
                error!("Cannot load scene `{}`:\n{}", path, e);
                scene_new()
            },
        },
        Err(e) => {
            error!("Cannot open file `{}`: {}", path, e);
            scene_new()
        },
    }
}

///
/// # Safety
/// `scene` must be a valid `Scene` pointer
/// `model` must be a valid `Model` pointer
#[no_mangle]
pub unsafe extern "C" fn scene_add_model(scene: *mut Scene, model: *const Model) {
    let scene = assert_ptr_mut(scene);
    let model = assert_ptr(model);

    scene.entities.push(entity![
        Transform::identity(),
        DefaultMaterial {
            diffuse_map: match Texture::new("/tmp/crate.png", None) {
                Ok(t) => t,
                Err(e) => {
                    error!("Cannot create texture `/tmp/crate.png`: {e}");
                    return;
                },
            },
            specular_map: match Texture::new("/tmp/crate_spec.png", None) {
                Ok(t) => t,
                Err(e) => {
                    error!("Cannot create texture `/tmp/crate_spec.png`: {e}");
                    return;
                },
            },
            ..Default::default()
        },
        model.clone()
    ]);

    debug!("Scene::add_model()");
}

///
/// # Safety
/// `scene` must be a valid `Scene` pointer
/// `path` must be a valid C string pointer
#[no_mangle]
pub unsafe extern "C" fn scene_save(scene: *const Scene, path: *const c_char) {
    let path = ptr_to_string(path);

    let scene = match ron::ser::to_string_pretty(
        assert_ptr(scene), 
        ron::ser::PrettyConfig::default()
    ){
        Ok(scene) => scene,
        Err(e) => {
            error!("Cannot save scene: {}", e);
            return;
        }
    };

    if let Err(e) = std::fs::write(path, scene) {
        error!("Cannot save scene: {}", e);
    }

    debug!("Scene::save()");
}

///
/// # Safety
/// `scene` must be a valid `Scene` pointer
#[no_mangle]
pub unsafe extern "C" fn scene_free(scene: *mut Scene) {
    free_ptr(scene);
    debug!("Scene::free()");
}