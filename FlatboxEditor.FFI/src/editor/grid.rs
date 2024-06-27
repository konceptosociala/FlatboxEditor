use flatbox_native_macro::native;
use serde::{Serialize, Deserialize};
use flatbox_core::{
    catch::CatchError, math::{glm, transform::Transform}
};
use flatbox_render::{
    pbr::{
        material::Material, 
        mesh::Mesh, 
        model::Model,
    }, 
    renderer::{
        RenderCommand, 
        PrepareModelCommand, 
        Renderer, 
        DrawModelCommand,
    }, 
    error::RenderError, 
    hal::shader::GraphicsPipeline,
};
use flatbox_assets::typetag;

use crate::NativeColor;

#[derive(Debug, Clone)]
pub struct Grid {
    model: Model,
    material: GridMaterial,
    transform: Transform,
}

impl Default for Grid {
    fn default() -> Self {
        Grid::new(10, 10, 10, NativeColor::new(253, 86, 54))
    }
}

#[native]
impl Grid {
    pub fn new(width: u32, height: u32, resolution: u32, color: NativeColor) -> Grid {
        let step_x = width as f32 / resolution as f32;
        let step_y = height as f32 / resolution as f32;

        let mut mesh = Mesh::empty();

        for x in 0..=resolution {
            for y in 0..=resolution {
                
            }
        }

        Grid {
            model: Model::new(mesh),
            material: GridMaterial::new(color),
            transform: Transform::identity(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridMaterial {
    pub color: glm::Vec3,
}

impl GridMaterial {
    pub fn new(color: NativeColor) -> GridMaterial {
        GridMaterial {
            color: color.into()
        }
    }
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
        pipeline.set_vec3("gridColor", &self.color);
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