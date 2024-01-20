use std::ffi::c_char;
use flatbox_core::{logger::{error, debug}, math::transform::Transform};
use flatbox_assets::{ron, scene::{Scene as NativeScene, SerializableEntity}, entity};
use flatbox_render::{pbr::{material::DefaultMaterial, model::Model, texture::Texture}, include_texture};
use native_macro::native;

#[derive(Default)]
pub struct Scene(NativeScene);

#[native]
impl Scene {
    pub fn new() -> Scene {
        debug!("Scene1::new()");
        Scene::default()
    }

    pub fn open(path: &str) -> Scene {
        debug!("Scene::open()");
        match std::fs::read_to_string(path) {
            Ok(ser) => match ron::from_str(&ser) {
                Ok(scene) => Scene(scene),
                Err(e) => {
                    error!("Cannot load scene `{}`:\n{}", path, e);
                    return Scene::scene_new();
                },
            },
            Err(e) => {
                error!("Cannot open file `{}`: {}", path, e);
                return Scene::scene_new();
            },
        }
    }

    pub fn add_model(scene: Scene, model: Model) {    
        scene.0.entities.push(entity![
            Transform::identity(),
            DefaultMaterial {
                diffuse_map: include_texture!("../assets/textures/dev.png"),
                specular_map: Texture::default(),
                ..Default::default()
            },
            model.clone()
        ]);
    
        debug!("Scene::add_model()");
    }

    pub fn scene_save(scene: Scene, path: &str) {
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