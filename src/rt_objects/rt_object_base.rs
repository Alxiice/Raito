/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines objects (geometries or lights) 
/// =====================================================

use crate::rt_types::*;
use crate::rt_ray::*;
use crate::rt_shader_globals::*;
use crate::rt_shaders::*;

use self::rt_shader_base::RtShader;


pub struct ObjectParams {
    pub name: String,
    pub shader: Box<dyn RtShader>
}

pub trait RtObject {
    /// Get object parameters
    fn getObjectParams(&self) -> &ObjectParams;
    
    /// Get object name
    fn get_name(&self) -> String {
        self.getObjectParams().name.clone()
    }

    /// Get shader attached to the object
    fn get_shader(&self) -> &Box<dyn RtShader> {
        &self.getObjectParams().shader
    }
    
    /// Get intersection point
    fn get_intersection_point(&self, ray: &RtRay) -> Option<RtPoint3>;

    /// Get object normal
    fn get_normal(&self, point: &RtPoint3) -> RtVec3;

    /// Compute ray-to-object intersection
    fn intersect(&self, ray: &RtRay) -> Option<RtShaderGlobals>;
}
