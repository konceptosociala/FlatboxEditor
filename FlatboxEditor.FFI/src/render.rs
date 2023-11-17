use std::ffi::{c_char, c_void, CString};
use flatbox_assets::scene::Scene;
use flatbox_render::{renderer::{Renderer, ClearCommand, RenderCameraCommand}, pbr::material::DefaultMaterial};

use crate::Camera;

pub type GlInitFunctionFFI = extern fn(*const c_char) -> *const c_void;

#[no_mangle]
pub extern "C" fn renderer_init(init_function: GlInitFunctionFFI) -> *mut Renderer {
    Box::into_raw(Box::new(Renderer::init(|ptr| {
        let cstring = CString::new(ptr).unwrap();
        let ptr = cstring.as_ptr() as *const _;
        init_function(ptr)
    })))
}

/// # Safety
/// `renderer` must be a valid `Renderer` pointer
/// `scene`  must be a valid `Renderer` pointer
#[no_mangle]
pub unsafe extern "C" fn renderer_render_scene(renderer: *mut Renderer, scene: *mut Scene) {
    let _renderer = {
        assert!(!renderer.is_null());
        &mut *renderer
    };

    let _scene = {
        assert!(!scene.is_null());
        &mut *scene
    };

    
    // for (_, (mut model, material, transform)) in &mut model_world.query::<(&mut Model, &M, &Transform)>() {
    //     renderer.execute(&mut PrepareModelCommand::new(&mut model, material))?;
    //     renderer.execute(&mut DrawModelCommand::new(&model, material, transform))?;
    // }

    todo!("Rendering scene");
}

/// # Safety
/// `renderer` must be a valid `Renderer` pointer
/// `scene`  must be a valid `Renderer` pointer
#[no_mangle]
pub unsafe extern "C" fn renderer_bind_camera(renderer: *mut Renderer, camera: *mut Camera) {
    let renderer = {
        assert!(!renderer.is_null());
        &mut *renderer
    };

    let camera = {
        assert!(!camera.is_null());
        &mut *camera
    };

    if let Err(e) = renderer.execute(&mut RenderCameraCommand::<DefaultMaterial>::new(
        &mut camera.inner, 
        &camera.transform
    )){
        eprintln!("Cannot bind camera: {e}");
    };

}

///
/// # Safety
/// `renderer` must be a valid `Renderer` pointer
#[no_mangle]
pub unsafe extern "C" fn renderer_free(renderer: *mut Renderer) {
    if renderer.is_null() {
        return;
    }
    
    let _ = Box::from_raw(renderer);
}

///
/// # Safety
/// `renderer` must be a valid `Renderer` pointer
#[no_mangle]
pub unsafe extern "C" fn renderer_clear(
    renderer: *mut Renderer, 
    r: f32, 
    g: f32, 
    b: f32
){
    let renderer = {
        assert!(!renderer.is_null());
        &mut *renderer
    };

    renderer.execute(&mut ClearCommand(r, g, b)).unwrap();
}