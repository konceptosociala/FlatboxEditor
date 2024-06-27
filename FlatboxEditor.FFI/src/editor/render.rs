use flatbox_core::{
    math::transform::Transform as NativeTransform, 
    catch::CatchError,
};
use flatbox_render::{
    macros::c_string,
    renderer::{Renderer as NativeRenderer, ClearCommand, RenderCameraCommand, PrepareModelCommand, DrawModelCommand}, 
    pbr::{
        material::DefaultMaterial, 
        model::Model as NativeModel,
        camera::Camera as NativeCamera,
    },
};
use flatbox_native_macro::native;

use crate::{is_shared_type, shared_mut, shared_ref, Camera, DrawGridCommand, Grid, GridMaterial, Scene, ToNative};
use crate::GlInitFunctionFFI;

pub struct Renderer(NativeRenderer);

#[native]
impl Renderer {
    pub fn init(init_function: GlInitFunctionFFI) -> Renderer {
        let mut renderer = NativeRenderer::init(|ptr| {
            let ptr = c_string!(ptr);
            init_function(ptr.as_ptr())
        });
    
        renderer.bind_material::<DefaultMaterial>();
        renderer.bind_material::<GridMaterial>();

        Renderer(renderer)
    }

    pub fn render_scene(renderer: &mut Renderer, scene: &mut Scene) {
        for entity in &mut scene.native_scene().lock().entities {
            let mut model = None;
            let mut material = None;
            let mut transform = None;
    
            for component in &entity.components {
                if is_shared_type!(component => NativeModel) { 
                    model = Some(component.clone());
                }

                if is_shared_type!(component => DefaultMaterial) { 
                    material = Some(component.clone());
                }

                if is_shared_type!(component => NativeTransform) {
                    transform = Some(component.clone());
                }
            }
    
            if let (Some(model), Some(material), Some(transform)) = (model, material, transform) {    
                renderer.0.execute(&mut PrepareModelCommand::new(
                    shared_mut!(model => NativeModel), 
                    shared_ref!(material => DefaultMaterial),
                )).catch();
    
                renderer.0.execute(&mut DrawModelCommand::new(
                    shared_ref!(model => NativeModel),
                    shared_ref!(material => DefaultMaterial), 
                    shared_ref!(transform => NativeTransform),
                )).catch();
            }
        }
    }

    pub fn render_grid(renderer: &mut Renderer, grid: &mut Grid) {
        renderer.0.execute(&mut DrawGridCommand::new(grid)).catch();
    }

    pub fn bind_camera(renderer: &mut Renderer, camera: &mut Camera) {    
        renderer.0.execute(&mut RenderCameraCommand::<DefaultMaterial>::new(
            shared_mut!(camera.native() => NativeCamera), 
            shared_ref!(camera.transform().native() => NativeTransform),
        )).catch();
    }

    pub fn bind_camera_grid(renderer: &mut Renderer, camera: &mut Camera) {    
        renderer.0.execute(&mut RenderCameraCommand::<GridMaterial>::new(
            shared_mut!(camera.native() => NativeCamera), 
            shared_ref!(camera.transform().native() => NativeTransform),
        )).catch();
    }

    pub fn clear(renderer: &mut Renderer, r: f32, g: f32, b: f32){
        renderer.0.execute(&mut ClearCommand(r, g, b)).catch();
    }
}