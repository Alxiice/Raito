/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines a render scene 
/// =====================================================

use std::io::Read;
use std::collections::HashMap;
use std::io::BufReader;
use std::fs::File;
use std::path::Display;
use std::str::FromStr;
use std::any::{Any, TypeId};
use itertools::Itertools;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use log::{info, error};

use crate::rt_camera::RtCamera;
use crate::rt_objects::rt_object_base::*;
use crate::rt_shaders::rt_shader_base::RtShader;
use crate::{RtPoint3, RtRGBA, RtVec3};

// TODO : for the placeholder geometry !
//        should be imported automatically
use crate::rt_shaders::lambert::LambertShader;
use crate::rt_shaders::metal::Metal;
use crate::rt_shaders::glass::Glass;
use crate::rt_objects::rt_geometries::RtSphere;


// ========================================
//  Render settings
// ========================================

pub struct RtRenderSettings {
    pub render_spp: u8,
    pub max_bounces: u8,
}

impl RtRenderSettings {
    pub fn new(render_spp: u8, max_bounces: u8) -> Self {
        Self { render_spp, max_bounces }
    }

    pub fn update(&mut self, render_spp: u8, max_bounces: u8) {
        self.render_spp = render_spp;
        self.max_bounces = max_bounces;
    }
}


// ========================================
//  RtScene is the scene object
//  that can be passed everywhere and 
//  used for intersections
// ========================================

pub struct RtScene {
    pub settings: RtRenderSettings,
    camera: RtCamera,
    shapes: RtObjectList,
    lights: RtObjectList,
}

impl RtScene {
    pub fn new(settings: RtRenderSettings, camera: RtCamera) -> Self {
        Self {
            settings, camera,
            shapes: RtObjectList::new(),
            lights: RtObjectList::new(),
        }
    }
    
    // === SETTERS ===

    pub fn set_settings(&mut self, settings: RtRenderSettings) {
        self.settings = settings;
    }

    pub fn set_camera(&mut self, camera: RtCamera) {
        self.camera = camera;
    }

    pub fn add_shape(&mut self, shape: Box<dyn RtObject>) {
        self.shapes.add_object(shape)
    }

    pub fn add_light(&mut self, light: Box<dyn RtObject>) {
        self.lights.add_object(light)
    }

    // === GETTERS ===

    pub fn get_camera(&self) -> &RtCamera {
        &self.camera
    }

    pub fn list_shapes(&self) -> &Vec<Box<dyn RtObject>> {
        self.shapes.list_objects()
    }

    pub fn list_lights(&self) -> &Vec<Box<dyn RtObject>> {
        self.lights.list_objects()
    }
}


// ========================================
//  XML scene format
// ========================================

const XML_ELEMENTS_LIST: &[&str] = &[
    "render_settings",
    "camera",
    "shader",
    "shape",
];

struct XMLParam {
    param_type: String,
    param_name: String,
    param_values: HashMap<String, String>
}

impl XMLParam {
    fn new(param_type: String, param_name: String) -> Self {
        Self { param_type, param_name, param_values: HashMap::new() }
    }

    fn add_value(&mut self, key: String, value: String) {
        // TODO : handle the case where the key is already in the param hashmap
        self.param_values.entry(key).or_insert(value);
    }

    fn get_value(&self, k: &str) -> String {
        if self.param_values.contains_key(k) {
            self.param_values[k].clone()
        } else {
            panic!("Key {} not in the parameter", k);
        }
    }

    fn extract_param_value<T: FromStr + std::fmt::Debug>(&self, k: &str, extract_type: TypeId) -> T {
        let str_value = self.get_value(k);
        let parsed_value: T = match str_value.parse() {
            Ok(v) => v,
            Err(_) => panic!("Value {}:{} cannot be parsed to {:?}", k, str_value, extract_type)
        };
        parsed_value
    }

    fn get_u8(&self) -> Result<u8, String> {
        if self.param_type != "int" {
            Err(format!("Parameter type is {}, not int", self.param_type))
        } else {
            Ok(self.extract_param_value("value", TypeId::of::<u8>()))
        }
    }

    fn get_u16(&self) -> Result<u16, String> {
        if self.param_type != "int" {
            Err(format!("Parameter type is {}, not int", self.param_type))
        } else {
            Ok(self.extract_param_value("value", TypeId::of::<u16>()))
        }
    }
    
    fn get_f32(&self) -> Result<f32, String> {
        if self.param_type != "float" {
            Err(format!("Parameter type is {}, not float", self.param_type))
        } else {
            Ok(self.extract_param_value("value", TypeId::of::<f32>()))
        }
    }

    fn get_rgb(&self) -> Result<RtRGBA, String> {
        if self.param_type != "rgb" {
            Err(format!("Parameter type is {}, not rgb", self.param_type))
        } else {
            Ok(RtRGBA::new(
                self.extract_param_value("r", TypeId::of::<f32>()), 
                self.extract_param_value("g", TypeId::of::<f32>()), 
                self.extract_param_value("b", TypeId::of::<f32>()), 
            ))
        }
    }

    fn get_point(&self) -> Result<RtPoint3, String> {
        if self.param_type != "point" {
            Err(format!("Parameter type is {}, not point", self.param_type))
        } else {
            Ok(RtPoint3::new(
                self.extract_param_value("x", TypeId::of::<f32>()), 
                self.extract_param_value("y", TypeId::of::<f32>()), 
                self.extract_param_value("z", TypeId::of::<f32>()), 
            ))
        }
    }

    fn get_vec3(&self) -> Result<RtVec3, String> {
        if self.param_type != "vec3" {
            Err(format!("Parameter type is {}, not vec3", self.param_type))
        } else {
            Ok(RtVec3::new(
                self.extract_param_value("x", TypeId::of::<f32>()), 
                self.extract_param_value("y", TypeId::of::<f32>()), 
                self.extract_param_value("z", TypeId::of::<f32>()), 
            ))
        }
    }
}

impl std::fmt::Display for XMLParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO : learn how to do padding
        let mut value_str = String::new();
        let mut remaining_items = self.param_values.len();
        for (key, value) in &self.param_values {
            value_str += format!("{}={}", key, value).as_str();
            remaining_items -= 1;
            if remaining_items > 0 {
                value_str += " ";
            }
        }
        write!(f, "<Param {} ({}) : {}>", self.param_name, self.param_type, value_str)
    }
}

struct XMLSceneElement {
    name: String,                   // "shader", "shape", "camera" ...
    identifier: Option<String>,     // unique identifier (e.g. "root/geo/shapes/my_shape")
    element_type: Option<String>,   // sub-type (e.g. shader->glass, shape->sphere)
    parameters: Vec<XMLParam>       // list of parameters
}

impl std::fmt::Display for XMLSceneElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut id_str = "None";
        if self.identifier.is_some() {
            id_str = self.identifier.as_ref().unwrap().as_str();
        }
        let mut type_str = "None";
        if self.element_type.is_some() {
            type_str = self.element_type.as_ref().unwrap().as_str();
        }
        write!(f, "<{} id={} type={}\n{}\n/>", self.name, id_str, type_str,
            self.parameters.iter().map(|x| { format!("  {}", x)}).join("\n")
        )
    }
}

impl XMLSceneElement {
    fn new(name: String, identifier: Option<String>, element_type: Option<String>) -> Self {
        Self { name, identifier, element_type, parameters: Vec::new() }
    }

    fn add_parameter(&mut self, parameter: XMLParam) {
        self.parameters.push(parameter);
    }
}

// We declare a XML scene as a list of XML scene elements
struct XMLScene(Vec<XMLSceneElement>);

impl std::fmt::Display for XMLScene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Scene\n{}\n/>", 
            self.0.iter().map(|x| { 
                x.to_string().split("\n").map(|y| {
                    format!("  {}", y)
                }).join("\n")
                // format!("{}", x.to_string().split("\n")) 
            }).join("\n")
        )
    }
}

impl XMLScene {
    fn new() -> Self {
        Self { 0: Vec::new() }
    }

    fn add_tag(&mut self, tag: XMLSceneElement) {
        self.0.push(tag);
    }

    fn process_attributes(node: BytesStart<'_>) -> Result<XMLParam, ()> {
        let mut pName: Option<String> = None;
        let mut pType: Option<String> = None;
        // Find name and type
        for attribute in node.attributes() {
            let mut key = String::new();
            _ = attribute.clone().unwrap().key.0.read_to_string(&mut key);
            let value = attribute.clone().unwrap().value;
            let value = std::str::from_utf8(value.as_ref()).unwrap();
            if key == "type" {
                pType = Some(String::from(value));
            } else if key == "name" {
                pName = Some(String::from(value));
            }
        }
        if pType.is_none() || pName.is_none() {
            // Missing info
            error!("Could not parse param !");
            return Err(())
        }
        let mut xml_param = XMLParam::new(pType.unwrap(), pName.unwrap());
        for attribute in node.attributes() {
            let mut key = String::new();
            _ = attribute.clone().unwrap().key.0.read_to_string(&mut key);
            let value = attribute.clone().unwrap().value;
            let value = std::str::from_utf8(value.as_ref()).unwrap();
            if key == "type" || key == "name" { continue };
            xml_param.add_value(key, String::from(value));
        }
        info!("    Parse param : {}", xml_param);
        Ok(xml_param)
    }

    fn get_tag(tag_name: String, node: &BytesStart<'_>) -> Option<XMLSceneElement> {
        // Check the tag is registered
        if XML_ELEMENTS_LIST.contains(&tag_name.as_str()) {
            info!(" +Parse tag {}", tag_name);
        } else {
            error!(" +Unknown tag {}", tag_name);
            return None
        }
        // Get tag header info
        let mut identifier = None;
        let mut element_type = None;
        for attribute in node.attributes() {
            let mut key = String::new();
            _ = attribute.clone().unwrap().key.0.read_to_string(&mut key);
            let value = attribute.clone().unwrap().value;
            let value = std::str::from_utf8(value.as_ref()).unwrap();
            match key.as_str() {
                "name" => {
                    identifier = Some(String::from(value));
                },
                "type" => {
                    element_type = Some(String::from(value));
                }
                _ => {}
            }
        }
        Some(XMLSceneElement::new(tag_name, identifier, element_type))
    }

    fn parse_tag(node: BytesStart<'_>, reader: &mut Reader<BufReader<File>>) -> Option<Result<XMLSceneElement, String>> {
        let mut tag_name = String::new();
        node.name().as_ref().read_to_string(&mut tag_name);
        let mut xml_tag = Self::get_tag(tag_name.clone(), &node);

        let mut buffer = Vec::new();
        loop {
            match reader.read_event_into(&mut buffer) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => return None,  // Break
                Ok(Event::Start(e)) => {
                    // New tag group starting here
                    panic!("Imbricated XML tags : not handled");
                }
                Ok(Event::Empty(e)) => {
                    if xml_tag.is_none() {
                        continue;       
                    }
                    match e.name().as_ref() {
                        b"parameter" => {
                            let param = Self::process_attributes(e);
                            if param.is_ok() {
                                xml_tag.as_mut().unwrap().add_parameter(param.unwrap());
                            }
                        },
                        _ => { panic!("unknown tag {:?}", e.name()) },
                    }
                }
                Ok(Event::End(e)) => {
                    let mut end_tag_name = String::new();
                    node.name().as_ref().read_to_string(&mut end_tag_name);
                    if end_tag_name.as_bytes() == tag_name.as_bytes() {
                        if xml_tag.is_some() {
                            info!(" -Closing tag {}", end_tag_name);
                            return Some(Ok(xml_tag.unwrap()))
                        } else {
                            info!(" -Closing unknown tag {}", end_tag_name);
                            return Some(Err(String::from(format!("Unknown tag {}", end_tag_name))))
                        }
                    } else {
                        panic!("Closing wrong tag {}", end_tag_name);
                    }
                }
                // Ignore other events
                Ok(_) => (),
            }
            // clear buffer to prevent memory leak
            buffer.clear();
        }
    }

    fn parse(path: &str) -> XMLScene {
        let mut xml_scene = Self::new();

        let mut reader = Reader::from_file(path).unwrap();
        let mut buffer = Vec::new();
        let mut current_xml_element: Option<XMLSceneElement> = None;

        loop {
            match reader.read_event_into(&mut buffer) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {
                    match e.name().as_ref() {
                        // Scene
                        b"scene" => { info!("Tag : scene") },
                        _ => {
                            let tag = Self::parse_tag(e, &mut reader);
                            if tag.is_none() { break } else if tag.as_ref().unwrap().is_ok() {
                                xml_scene.add_tag(tag.unwrap().unwrap());
                            }
                        }
                    }
                }
                Ok(Event::End(e)) => {
                    match e.name().as_ref() {
                        b"scene" => {
                            // Scene parsing is finished
                        },
                        _ => { panic!("Error while parsing the scene {:?}", e.name()) }
                    }
                }
                Ok(Event::Empty(e)) => { panic!("Error while parsing the scene {:?}", e.name()) }
                // Other Events are not important for us
                Ok(_) => (),
            }
    
            // clear buffer to prevent memory leak
            buffer.clear();
        }

        // Return scene
        xml_scene
    }

    fn get_settings(&self) -> RtRenderSettings {
        for el in &self.0 {
            if el.name == "render_settings" {
                let mut p_spp: u8 = 1;
                let mut p_bounces: u8 = 1;
                for p in &el.parameters {
                    if p.param_name == "spp" && p.get_u8().is_ok() {
                        p_spp = p.get_u8().unwrap();
                    }
                    if p.param_name == "max_bounces" && p.get_u8().is_ok() {
                        p_bounces = p.get_u8().unwrap();
                    }
                }
                return RtRenderSettings::new(p_spp, p_bounces)
            }
        }
        panic!("No render settings found in the scene !")
    }

    fn get_camera(&self) -> RtCamera {
        for el in &self.0 {
            if el.name == "camera" {
                // Camera found !
                let mut vfov: f32 = 30.0;
                let mut lookfrom = RtPoint3::default();
                let mut lookat = RtPoint3::default();
                for p in &el.parameters {
                    if p.param_name == "v_fov" && p.get_f32().is_ok() {
                        vfov = p.get_f32().unwrap();
                    }
                    if p.param_name == "look_from" && p.get_point().is_ok() {
                        lookfrom = p.get_point().unwrap();
                    }
                    if p.param_name == "look_at" && p.get_point().is_ok() {
                        lookat = p.get_point().unwrap();
                    }
                }
                // Create and return camera
                return RtCamera::new(
                    1.0, 400, 
                    vfov, 
                    lookfrom, 
                    lookat, 
                    RtVec3::new(0.0, 1.0, 0.0)
                )
            }
        }
        panic!("No camera found in the scene !")
    }

    fn get_shaders(&self) -> HashMap<String, Box<dyn RtShader>> {
        // shader_id -> shader
        let shaders: HashMap<String, Box<dyn RtShader>> = HashMap::new();
        for el in &self.0 {
            if el.name == "shader" {

            }
        }

        shaders
    }


    fn as_rt_scene(&self) -> Result<RtScene, &str> {
        // Create the scene
        let settings = self.get_settings();
        let camera = self.get_camera();
        let mut scene = RtScene::new(settings, camera);
        // Add geometry in the scene
        error!("Geometry not implemented");
        // TODO

        // Placeholder geometry
        scene.add_shape(Box::new(RtSphere { 
            object_params: ObjectParams::new(
                String::from(""), String::from(""),
                Box::new(LambertShader {
                    color: RtRGBA::from_rgb(0.5, 0.5, 0.5)
                })
            ),
            center: RtPoint3::new(0.0, -1000.0, 0.0),
            radius: 1000.0
        }));
        scene.add_shape(Box::new(RtSphere { 
            object_params: ObjectParams::new(
                String::from(""), String::from(""),
                Box::new(LambertShader {
                    color: RtRGBA::from_rgb(0.4, 0.2, 0.1)
                })
            ),
            center: RtPoint3::new(-4.0, 1.0, 0.0),
            radius: 1.0
        }));
        scene.add_shape(Box::new(RtSphere { 
            object_params: ObjectParams::new(
                String::from(""), String::from(""),
                Box::new(Glass {
                    ior: 1.5
                })
            ),
            center: RtPoint3::new(0.0, 1.0, 0.0),
            radius: 1.0
        }));
        scene.add_shape(Box::new(RtSphere { 
            object_params: ObjectParams::new(
                String::from(""), String::from(""),
                Box::new(Metal {
                    color: RtRGBA::from_rgb(0.7, 0.6, 0.5),
                    fuzz: 0.0
                })
            ),
            center: RtPoint3::new(4.0, 1.0, 0.0),
            radius: 1.0
        }));
        // />

        // Return the scene
        Ok(scene)
    }
}


pub fn open_xml_scene(path: &str) -> Option<RtScene> {
    info!("Opening XML render scene : {path}");
    let now = std::time::Instant::now();

    let xml_scene = XMLScene::parse(path);
    info!("Scene : \n{}", xml_scene);
    let scene = xml_scene.as_rt_scene();
    if scene.is_err() {
        error!("Could not parse scene {path} : {:?}", scene.err());
        return None;
    }

    info!("> Scene took {} sec to open", now.elapsed().as_secs_f64());
    Some(scene.unwrap())
}
