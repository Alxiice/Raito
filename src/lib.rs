#![warn(clippy::all, rust_2018_idioms)]
#![allow(non_snake_case)]

/// =====================================================
///                    Raito Render
/// 
/// Module authors : 
/// - Alice Sonolet <alice.sonolet@gmail.com>
/// 
/// Module description :
///   Defines render engine library 
/// =====================================================

mod rt_render;
pub use rt_render::*;
mod rt_types;
pub use rt_types::*;
mod rt_ray;
pub use rt_ray::*;
mod rt_camera;
pub use rt_camera::*;
mod rt_geometry;
pub use rt_geometry::*;
mod rt_shader_globals;
pub use rt_shader_globals::*;
