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
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use log::{info, error};

use crate::rt_camera::RtCamera;
use crate::rt_objects::rt_object_base::*;


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
    // "render_settings",
    // "camera"
    "shape"
];

struct XMLParam {
    param_type: String,
    param_values: HashMap<String, String>
}

impl XMLParam {
    fn new(param_type: String) -> Self {
        Self { param_type, param_values: HashMap::new() }
    }

    fn add_value(&mut self, key: String, value: String) {
        // TODO : handle the case where the key is already in the param hashmap
        self.param_values.entry(key).or_insert(value);
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
        write!(f, "<Param ({}) : {}>", self.param_type, value_str)
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
        write!(f, "<{} id={} type={}>", self.name, id_str, type_str)
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

impl XMLScene {
    fn new() -> Self {
        Self { 0: Vec::new() }
    }

    fn process_attributes(node: BytesStart<'_>) {
        let mut xml_param: Option<XMLParam> = None;
        for attribute in node.attributes() {
            let mut key = String::new();
            _ = attribute.clone().unwrap().key.0.read_to_string(&mut key);
            if key == "type" {
                let value = attribute.clone().unwrap().value;
                let value = std::str::from_utf8(value.as_ref()).unwrap();
                xml_param = Some(XMLParam::new(String::from(value)));
                break;
            }
        }
        if xml_param.is_none() {
            error!("Could not parse param !");
            // TODO : return Err
        } else {
            let mut params = xml_param.unwrap();
            for attribute in node.attributes() {
                let mut key = String::new();
                _ = attribute.clone().unwrap().key.0.read_to_string(&mut key);
                let value = attribute.clone().unwrap().value;
                let value = std::str::from_utf8(value.as_ref()).unwrap();
                params.add_value(key, String::from(value));
            }
            info!("    -> {}", params);
            // TODO : return Ok(xml_param)
        }
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

    fn parse_tag(node: BytesStart<'_>, reader: &mut Reader<BufReader<File>>) -> Option<()> {
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
                            Self::process_attributes(e);
                        },
                        _ => { panic!("unknown tag {:?}", e.name()) },
                    }
                }
                Ok(Event::End(e)) => {
                    let mut end_tag_name = String::new();
                    node.name().as_ref().read_to_string(&mut end_tag_name);
                    if end_tag_name.as_bytes() == tag_name.as_bytes() {
                        if xml_tag.is_some() {
                            info!(" -Closing tag {} -> {}", end_tag_name, xml_tag.unwrap());
                        } else {
                            info!(" -Closing unknown tag {}", end_tag_name);
                        }
                        return Some(())
                    } else {
                        panic!("Closing wrong tag {}", end_tag_name);
                    }
                }
                Ok(_) => (),  // Ignore other events
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
                            if tag.is_none() { break }
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

    fn as_rt_scene(&self) -> Result<RtScene, &str> {
        // TODO : implement
        Err("Not implemented yet")
    }
}


pub fn open_xml_scene(path: &str) -> Option<RtScene> {
    info!("Opening XML render scene : {path}");
    let now = std::time::Instant::now();

    let xml_scene = XMLScene::parse(path);
    let scene = xml_scene.as_rt_scene();
    if scene.is_err() {
        error!("Could not parse scene {path} : {:?}", scene.err());
        return None;
    }

    info!("> Scene took {} sec to open", now.elapsed().as_secs_f64());
    Some(scene.unwrap())
}
