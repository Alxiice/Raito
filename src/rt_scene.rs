/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines a render scene 
/// =====================================================

use crate::rt_types::*;
use crate::rt_objects::*;
use crate::rt_objects::rt_object_base::*;
use crate::rt_shaders::*;

use self::lambert::LambertShader;
use self::lightShader::LightShader;
use self::rt_lights::RtPointLight;
use self::rt_geometries::RtSphere;


/// Describes a render scene
pub struct RtScene {
    // Camera params
    pub camera_fov: f32,
    // Sphere params
    pub sphere: RtSphere,
    // Light params
    pub light: RtPointLight,
}

impl Default for RtScene {
    fn default() -> Self {
        Self {
            // Camera params
            camera_fov: 1.0,
            // Sphere params
            sphere: RtSphere { 
                object_params: ObjectParams { name: String::default(), shader: Box::new(DEFAULT_SHADER) },
                center: RtPoint3::default(),
                radius: 1.0
            },
            // Light params
            light: RtPointLight {
                object_params: ObjectParams { name: String::default(), shader: Box::new(DEFAULT_LIGHT) },
                center: RtPoint3::default(),
                radius: 1.0
            }
        }
    }
}

impl RtScene {
    /// Update scene parameters
    pub fn new(camera_fov: f32, 
               sphere_color: RtRGBA,
               sphere_center: RtPoint3,
               sphere_radius: f32,
               light_center: RtPoint3,
               light_radius: f32,
               light_color: RtRGBA,
               light_intensity: f32) -> Self
    {
        Self {
            camera_fov,
            sphere: RtSphere {
                object_params: ObjectParams {
                    name: String::from("sphere"), 
                    shader: Box::new(LambertShader{ color: sphere_color })
                },
                center: sphere_center,
                radius: sphere_radius
            },
            light: RtPointLight {
                object_params: ObjectParams {
                    name: String::from("light"),
                    shader: Box::new(LightShader { 
                        color: light_color, intensity: light_intensity 
                    })
                },
                center: light_center,
                radius: light_radius
            }
        }
    }

    // TODO : from one to multiple objects
    /// Iterate on the scene objects
    pub fn get_scene_geometry(&self) -> Box<& dyn RtObject> {
        Box::new(&self.sphere)
    }

    pub fn get_scene_light(&self) -> Box<& dyn RtObject> {
        Box::new(&self.light)
    }
}
