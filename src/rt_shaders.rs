/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines shaders module
/// =====================================================

pub mod rt_shader_base;
pub mod staticColor;
pub mod stateVector;
pub mod lambert;
pub mod lightShader;
pub mod metal;
pub mod glass;

use staticColor::StaticColorShader;
use stateVector::StateVectorShader;
use lambert::LambertShader;
use metal::Metal;
use glass::Glass;
use lightShader::LightShader;

use crate::RtRGBA;

pub enum RtSurfaceShadersTypes {
    StaticColor(StaticColorShader),
    Normal(StateVectorShader),
    Lambert(LambertShader),
    Glass(Glass),
    Metal(Metal),
}

pub enum RtLightShadersTypes {
    LightShader(LightShader)
}

pub const DEFAULT_SHADER: StaticColorShader = StaticColorShader { color: RtRGBA::ERRCOLOR };
pub const DEFAULT_LIGHT: LightShader = LightShader { color: RtRGBA::WHITE, intensity: 1.0 };
