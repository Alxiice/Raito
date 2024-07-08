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


// ========================================
//  Object parameters
// ========================================

pub struct ObjectParams {
    pub name: String,
    pub object_type: String,
    pub shader: Box<dyn RtShader>
}

impl ObjectParams {
    pub fn new(name: String, object_type: String, shader: Box<dyn RtShader>) -> ObjectParams {
        ObjectParams {
            name,
            object_type,
            shader
        }
    }
}


// ========================================
//  Object
// ========================================

pub trait RtObject: Send + Sync {
    /// Get object parameters
    fn getObjectParams(&self) -> &ObjectParams;
    
    /// Get object name
    fn get_name(&self) -> String {
        self.getObjectParams().name.clone()
    }

    /// Get object type
    fn get_type(&self) -> String {
        self.getObjectParams().object_type.clone()
    }

    /// Get shader attached to the object
    fn get_shader(&self) -> &Box<dyn RtShader> {
        &self.getObjectParams().shader
    }
    
    /// Get intersection point
    fn get_intersection(&self, ray: &RtRay) -> Option<RtRayHit>;

    /// Get object normal
    fn get_normal(&self, point: &RtPoint3) -> RtVec3;

    /// Get shader globals
    fn get_sg(&self, ray: &RtRay, hit: &RtRayHit) -> RtShaderGlobals {
        let mut sg = RtShaderGlobals::from_intersection(
            ray, 
            self.get_name(),
            hit.P.unwrap()
        );
        sg.N = self.get_normal(&sg.P);
        sg
    }

    /// Compute ray-to-object intersection
    fn intersect(&self, ray: &RtRay) -> Option<RtShaderGlobals> {
        let intersection_point = self.get_intersection(ray);
        if !intersection_point.is_some() {
            return None
        }
        Some(self.get_sg(ray, &intersection_point.unwrap()))
    }
}


// ========================================
//  Object list
// ========================================

// Define type linked to a list of object
pub struct RtObjectList(Vec<Box<dyn RtObject>>);
// Or : pub struct RtObjectList<'a>(Vec<Box<&'a dyn RtObject>>);

impl RtObjectList {
    pub fn new() -> Self {
        Self {
            0: Vec::new()
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn add_object(&mut self, shape: Box<dyn RtObject>) {
        self.0.push(shape);
    }

    pub fn list_objects(&self) -> &Vec<Box<dyn RtObject>> {
        &self.0
    }
}
