use flatbox_core::math::glm;
use serde::{Serialize, Deserialize};

pub mod camera;
pub mod grid;
pub mod model;
pub mod render;
pub mod scene;

pub use camera::*;
pub use grid::*;
pub use model::*;
pub use render::*;
pub use scene::*;

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct NativeColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<NativeColor> for glm::Vec3 {
    fn from(color: NativeColor) -> Self {
        glm::vec3(
            (color.r / 255) as f32, 
            (color.g / 255) as f32, 
            (color.b / 255) as f32
        )
    }
}