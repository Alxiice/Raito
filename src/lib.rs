#![warn(clippy::all, rust_2018_idioms)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
// #![allow(unused)]

/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines render engine library 
/// =====================================================

pub mod rt_types;
pub use rt_types::*;
pub mod rt_ray;
pub use rt_ray::*;
pub mod rt_camera;
pub use rt_camera::*;

// Object module
pub mod rt_objects;
pub mod rt_shaders;

pub mod rt_scene;
pub use rt_scene::*;
pub mod rt_shader_globals;
pub use rt_shader_globals::*;
pub mod rt_render;
pub use rt_render::*;
pub mod rt_render_scene;
pub use rt_render_scene::*;
