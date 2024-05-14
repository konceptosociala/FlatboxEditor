use flatbox_core::{
    math::transform::Transform, 
    catch::CatchError, logger::debug,
};
use flatbox_render::{
    macros::c_string,
    renderer::{Renderer as NativeRenderer, ClearCommand, RenderCameraCommand, PrepareModelCommand, DrawModelCommand}, 
    pbr::{
        material::DefaultMaterial, 
        model::Model
    },
};
use flatbox_native_macro::native;

use crate::{Camera, DrawGridCommand, Grid, GridMaterial, Scene};
use crate::GlInitFunctionFFI;

pub struct Renderer(pub NativeRenderer);

#[native]
impl Renderer {
    pub fn init(init_function: GlInitFunctionFFI) -> Renderer {
        let mut renderer = NativeRenderer::init(|ptr| {
            let ptr = c_string!(ptr);
            init_function(ptr.as_ptr())
        });
    
        renderer.bind_material::<DefaultMaterial>();
        renderer.bind_material::<GridMaterial>();

        debug!("Renderer::init()");
        Renderer(renderer)
    }

    pub fn render_scene(renderer: &mut Renderer, scene: &mut Scene) {
        for entity in &mut scene.0.entities {
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
    
                renderer.0.execute(&mut PrepareModelCommand::new(
                    model.as_any_mut().downcast_mut::<Model>().unwrap(), 
                    material.as_any().downcast_ref::<DefaultMaterial>().unwrap(),
                )).catch();
    
                renderer.0.execute(&mut DrawModelCommand::new(
                    model.as_any().downcast_ref::<Model>().unwrap(), 
                    material.as_any().downcast_ref::<DefaultMaterial>().unwrap(), 
                    transform.as_any().downcast_ref::<Transform>().unwrap(),
                )).catch();
            }
        }
        debug!("Renderer::render_scene()");
    }

    pub fn render_grid(renderer: &mut Renderer, grid: &mut Grid) {
        renderer.0.execute(&mut DrawGridCommand::new(grid)).catch();
        debug!("Renderer::render_grid()");
    }

    pub fn bind_camera(renderer: &mut Renderer, camera: &mut Camera) {    
        renderer.0.execute(&mut RenderCameraCommand::<DefaultMaterial>::new(&mut camera.inner, &camera.transform)).catch();
        debug!("Renderer::bind_camera()");
    }

    pub fn bind_camera_grid(renderer: &mut Renderer, camera: &mut Camera) {    
        renderer.0.execute(&mut RenderCameraCommand::<GridMaterial>::new(&mut camera.inner, &camera.transform)).catch();
        debug!("Renderer::bind_camera_grid()");
    }

    pub fn clear(renderer: &mut Renderer, r: f32, g: f32, b: f32){
        renderer.0.execute(&mut ClearCommand(r, g, b)).catch();
        debug!("Renderer::clear()");
    }
}