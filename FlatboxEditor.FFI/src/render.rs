use std::ffi::{c_char, c_void, CString};
use flatbox_assets::scene::Scene;
use flatbox_core::{
    math::transform::Transform, 
    catch::CatchError, logger::debug,
};
use flatbox_render::{
    renderer::{Renderer, ClearCommand, RenderCameraCommand, PrepareModelCommand, DrawModelCommand}, 
    pbr::{
        material::DefaultMaterial, 
        model::Model
    },
};

use crate::{Camera, assert_ptr_mut, free_ptr};

pub type GlInitFunctionFFI = extern fn(*const c_char) -> *const c_void;

#[no_mangle]
pub extern "C" fn renderer_init(init_function: GlInitFunctionFFI) -> *mut Renderer {
    let mut renderer = Renderer::init(|ptr| {
        let cstring = CString::new(ptr).unwrap();
        let ptr = cstring.as_ptr() as *const _;
        init_function(ptr)
    });

    renderer.bind_material::<DefaultMaterial>();

    debug!("Renderer::init()");
    Box::into_raw(Box::new(renderer))
}

/// # Safety
/// `renderer` must be a valid `Renderer` pointer
/// `scene`  must be a valid `Scene` pointer
#[no_mangle]
pub unsafe extern "C" fn renderer_render_scene(renderer: *mut Renderer, scene: *mut Scene) {
    let renderer = assert_ptr_mut(renderer);
    let scene = assert_ptr_mut(scene);

    for entity in &scene.entities {
        let mut model = None;
        let mut material = None;
        let mut transform = None;

        for component in &entity.components {
            if component.lock().as_any_mut().downcast_mut::<Model>().is_some() { model = Some(component.clone()) }
            if component.lock().as_any().downcast_ref::<DefaultMaterial>().is_some() { material = Some(component.clone()) }
            if component.lock().as_any().downcast_ref::<Transform>().is_some() { transform = Some(component.clone()) }
        }

        if let (Some(model), Some(material), Some(transform)) = (model, material, transform) {
            let mut model = model.lock();
            let material = material.lock();
            let transform = transform.lock();

            renderer.execute(&mut PrepareModelCommand::new(
                model.as_any_mut().downcast_mut::<Model>().unwrap(), 
                material.as_any().downcast_ref::<DefaultMaterial>().unwrap(),
            )).catch();

            renderer.execute(&mut DrawModelCommand::new(
                model.as_any_mut().downcast_mut::<Model>().unwrap(), 
                material.as_any().downcast_ref::<DefaultMaterial>().unwrap(), 
                transform.as_any().downcast_ref::<Transform>().unwrap(),
            )).catch();
        }
    }
    debug!("Renderer::render_scene()");
}

/// # Safety
/// `renderer` must be a valid `Renderer` pointer
/// `scene`  must be a valid `Renderer` pointer
#[no_mangle]
pub unsafe extern "C" fn renderer_bind_camera(renderer: *mut Renderer, camera: *mut Camera) {
    let renderer = assert_ptr_mut(renderer);
    let camera = assert_ptr_mut(camera);

    renderer.execute(&mut RenderCameraCommand::<DefaultMaterial>::new(&mut camera.inner, &camera.transform)).catch();
    debug!("Renderer::bind_camera()");
}

///
/// # Safety
/// `renderer` must be a valid `Renderer` pointer
#[no_mangle]
pub unsafe extern "C" fn renderer_free(renderer: *mut Renderer) {
    free_ptr(renderer);
    debug!("Renderer::free()");
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
    let renderer = assert_ptr_mut(renderer);

    renderer.execute(&mut ClearCommand(r, g, b)).catch();
    debug!("Renderer::clear()");
}