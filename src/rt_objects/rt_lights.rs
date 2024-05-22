/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines objects (geometries or lights) 
/// =====================================================

use crate::rt_types::*;
use crate::rt_ray::*;
use crate::rt_shader_globals::*;
use crate::rt_objects::rt_object_base::*;


// ========================================
//  Define objects
// ========================================

pub struct RtSkydomeLight {
    pub color: RtRGBA,
    pub intensity: f32
}

impl RtSkydomeLight {
    const rt_type: &'static str = "<RtLight : Skydome>";
}

/// Point light
pub struct RtPointLight {
    pub color: RtRGBA,
    pub intensity: f32,
    pub center: RtPoint3,
    pub radius: f32
}

impl RtPointLight {
    const rt_type: &'static str = "<RtLight : Point>";
}

/// Defines light types
pub enum RtLightTypes {
    Skydome(RtSkydomeLight),
    Point(RtPointLight)
}


// ========================================
//  Implement object traits
// ========================================
