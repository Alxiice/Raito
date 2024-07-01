/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines a render scene 
/// =====================================================

use log::{info, error};
use std::io::Read;

use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};

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


fn process_attributes(node: BytesStart<'_>) {
    for attribute in node.attributes() {
        let mut key = String::new();
        _ = attribute.clone().unwrap().key.0.read_to_string(&mut key);
        let value = attribute.clone().unwrap().value;
        let value = std::str::from_utf8(value.as_ref()).unwrap();
        info!("  -> {:?}={:?}", key, value);
    }
}

fn parse_xml_file(path: &str) -> std::io::Result<()> {
    let mut reader = Reader::from_file(path).unwrap();

    let mut buffer = Vec::new();
    let mut _counter = 0;

    info!("Reading XML scene {path}");

    loop {
        match reader.read_event_into(&mut buffer) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    // Scene
                    b"scene" => {
                        info!("Found tag : scene");
                        process_attributes(e);
                    },
                    // Shapes
                    b"shader" => {
                        info!("Found tag : shader");
                        process_attributes(e);
                    },
                    
                    _ => {
                        error!("unknown tag {:?}", e.name());
                    },
                }
            }
            Ok(Event::Empty(e)) => {
                match e.name().as_ref() {
                    // Parameter
                    b"parameter" => {
                        info!("Found tag : parameter");
                        process_attributes(e);
                    },
                    _ => {
                        error!("unknown tag {:?}", e.name());
                    },
                }
            }
            Ok(Event::End(e)) => {
                match e.name().as_ref() {
                    _ => {
                        info!("closing tag {:?}", e.name());
                    },
                }
            }
            // Other Events are not important for us
            Ok(_) => _counter += 1,
        }

        // clear buffer to prevent memory leak
        buffer.clear();
    }

    info!("Scene loaded from XML file");

    Result::Ok(())
}


pub fn read_xml(path: &str) {
    // get_xml_stream(path);
    // let reader = Reader::from_file(path);

    let _ = parse_xml_file(path);
    return;

}
