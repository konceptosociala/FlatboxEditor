use std::ffi::c_char;
use flatbox_core::{logger::{error, debug}, math::transform::Transform};
use flatbox_assets::{ron, scene::{Scene as NativeScene, SerializableEntity}, entity};
use flatbox_render::{pbr::{material::DefaultMaterial, texture::Texture}, include_texture};
use native_macro::native;

use crate::Model;

#[derive(Default)]
pub struct Scene(pub NativeScene);

#[native]
impl Scene {
    pub fn new() -> Scene {
        debug!("Scene::new()");
        Scene::default()
    }

    pub fn open(path: &str) -> Scene {
        debug!("Scene::open()");
        match std::fs::read_to_string(path) {
            Ok(ser) => match ron::from_str(&ser) {
                Ok(scene) => Scene(scene),
                Err(e) => {
                    error!("Cannot load scene `{}`:\n{}", path, e);
                    Scene::new()
                },
            },
            Err(e) => {
                error!("Cannot open file `{}`: {}", path, e);
                Scene::new()
            },
        }
    }

    pub fn add_model(scene: &mut Scene, model: &Model) {    
        scene.0.entities.push(entity![
            Transform::identity(),
            DefaultMaterial {
                diffuse_map: include_texture!("../assets/textures/dev.png"),
                specular_map: Texture::default(),
                ..Default::default()
            },
            model.0.clone()
        ]);
    
        debug!("Scene::add_model()");
    }

    pub fn scene_save(scene: &Scene, path: &str) {
        let scene = match ron::ser::to_string_pretty(
            &scene.0,
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
}