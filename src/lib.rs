#![warn(clippy::all, rust_2018_idioms)]
/// =====================================================
///                    Raito Render
/// 
/// Module authors : 
/// - Alice Sonolet <alice.sonolet@gmail.com>
/// 
/// Module description :
///   Defines render engine library 
/// =====================================================

mod raito_render;
pub use raito_render::*;
mod rt_types;
pub use rt_types::*;
mod rt_ray;
pub use rt_ray::*;
mod rt_camera;
pub use rt_camera::*;
