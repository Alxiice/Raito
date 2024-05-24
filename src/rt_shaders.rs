/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines shaders module
/// =====================================================

pub mod rt_shader_base;
pub mod staticColor;
pub mod stateVector;

use staticColor::StaticColorShader;
use stateVector::StateVectorShader;

pub enum RtShadersTypes {
    StaticColor(StaticColorShader),
    Normal(StateVectorShader)
}
