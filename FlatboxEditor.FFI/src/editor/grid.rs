use serde::{Serialize, Deserialize};
use flatbox_core::{
    math::{transform::Transform, glm},
    catch::CatchError,
};
use flatbox_render::{
    pbr::{
        material::Material, 
        model::Model,
    }, 
    renderer::{
        RenderCommand, 
        PrepareModelCommand, 
        Renderer, 
        DrawModelCommand,
    }, 
    error::RenderError, hal::shader::GraphicsPipeline,
};
use flatbox_assets::typetag;

use crate::{free_ptr, NativeColor};

#[derive(Default, Debug, Clone)]
pub struct Grid {
    model: Model,
    material: GridMaterial,
    transform: Transform,
}

#[no_mangle]
pub extern "C" fn grid_new(
    _width: u32, 
    _height: u32, 
    _resolution: u32,
    _color: NativeColor,
) -> *mut Grid {
    Box::into_raw(Box::default())
}

///
/// # Safety
/// `grid` must be a valid `Grid` pointer
#[no_mangle]
pub unsafe extern "C" fn grid_free(grid: *mut Grid) {
    free_ptr(grid);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridMaterial {
    pub color: glm::Vec3,
}

impl Default for GridMaterial {
    fn default() -> Self {
        GridMaterial { color: glm::vec3(1.0, 1.0, 1.0) }
    }
}

#[typetag::serde]
impl Material for GridMaterial {
    fn vertex_shader() -> &'static str {
        include_str!("../shaders/grid.vs")
    }

    fn fragment_shader() -> &'static str {
        include_str!("../shaders/grid.fs")
    }

    fn setup_pipeline(&self, pipeline: &GraphicsPipeline) {
        pipeline.set_vec3("material", &self.color);
    }
}

#[derive(Debug)]
pub struct DrawGridCommand<'a> {
    grid: &'a mut Grid,
}

impl<'a> DrawGridCommand<'a> {
    pub fn new(grid: &mut Grid) -> DrawGridCommand {
        DrawGridCommand { grid }
    }
}

impl<'a> RenderCommand for DrawGridCommand<'a> {
    fn execute(&mut self, renderer: &mut Renderer) -> Result<(), RenderError> {
        renderer.execute(&mut PrepareModelCommand::new(
            &mut self.grid.model, 
            &self.grid.material,
        )).catch();

        renderer.execute(&mut DrawModelCommand::new(
            &self.grid.model,
            &self.grid.material,
            &self.grid.transform,
        )).catch();

        Ok(())
    }
}