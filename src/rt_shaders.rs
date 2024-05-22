/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines shaders
/// =====================================================

pub mod rt_shader_base;
pub mod staticColor;
pub mod stateVector;

use rt_shader_base::*;
use staticColor::*;
use stateVector::*;

pub enum RtShadersTypes {
    StaticColor(StaticColorShader),
    Normal(StateVectorShader)
}
