/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines a render scene 
/// =====================================================

use std::marker::PhantomData;

use crate::rt_types::*;
use crate::rt_camera::*;
use crate::rt_objects::*;
use crate::rt_objects::rt_object_base::*;
use crate::rt_shaders::*;

use self::staticColor::StaticColorShader;
use self::stateVector::StateVectorShader;
use self::rt_geometries::RtSphere;


/// Describes a render scene
pub struct RtScene<'a> {
    // Camera params
    pub camera_fov: f32,
    // Light params
    pub light_intensity: f32,
    pub light_color: RtRGBA,
    // Sphere params
    pub sphere_color: RtRGBA,
    pub sphere_center: RtPoint3,
    pub sphere_radius: f32,

    // To use lifetime
    _marker: PhantomData<&'a ()>,
}

impl<'a> Default for RtScene<'a> {
    fn default() -> Self {
        Self {
            // Camera params
            camera_fov: 1.0,
            // Light params
            light_intensity: 1.0,
            light_color: RtRGBA::default(),
            // Sphere params
            sphere_color: RtRGBA::default(),
            sphere_center: RtPoint3::default(),
            sphere_radius: 0.0,

            _marker: Default::default()
        }
    }
}

impl<'a> RtScene<'a> {
    /// Update scene parameters
    pub fn setup_scene(&mut self, 
        camera_fov: f32, 
        light_intensity: f32, 
        light_color: RtRGBA,
        sphere_color: RtRGBA,
        sphere_center: RtPoint3,
        sphere_radius: f32)
    {
        // Camera params
        self.camera_fov = camera_fov;
        // Light params
        self.light_intensity = light_intensity;
        self.light_color     = light_color;
        // Sphere params
        self.sphere_color  = sphere_color;
        self.sphere_center = sphere_center;
        self.sphere_radius = sphere_radius;
    }

    // TODO : from one to multiple objects
    /// Iterate on the scene objects
    pub fn get_scene_geometry(&self) -> Box<dyn RtObject<'static>> {
        let shader = StateVectorShader { output: "N".to_string() };
        // let shader = StaticColorShader { color: self.sphere_color };
        // Define geometry
        let sphere = RtSphere {
            object_params: ObjectParams {
                name: "sphere".to_string(),
                shader: Box::new(shader)
            },
            center: self.sphere_center,
            radius: self.sphere_radius
        };

        Box::new(sphere)
    }
}
