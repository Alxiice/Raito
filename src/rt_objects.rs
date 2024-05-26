/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines objects (geometries or lights) 
/// =====================================================

pub mod rt_object_base;
pub mod rt_geometries;
pub mod rt_lights;

use rt_geometries::*;
use rt_lights::*;


/// Defines object types
pub enum RtObjectTypes {
    RtNull(bool),
    Geometry(RtGeometryTypes),
    Light(RtLightTypes)
}
