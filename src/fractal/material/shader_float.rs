use bevy::{
    math::{uvec2, uvec4, vec2, DVec2, UVec2, UVec4, Vec2},
    reflect::TypePath,
    render::render_resource::{encase::internal::WriteInto, ShaderSize, ShaderType},
};
use std::fmt::Debug;

// I cannot for the life of me figure out how to
// implement ShaderType for f64
// I don't even know if it's legal or if it's gonna cause nasal demons
// on some obscure GPU driver
// So instead we're doing this shit

/// A trait for encoding floating point value into a supported shader type
///
/// This can either truncate to f32 or turn f64 into bit representation
pub trait EncodeShaderFloat: TypePath + Copy {
    const IS_DOUBLE_PRECISION: bool = false;
    type EncodedFp: ShaderType + ShaderSize + Send + Sync + Copy + Debug + WriteInto;
    type EncodedVec2: ShaderType + ShaderSize + Send + Sync + Copy + Debug + WriteInto;

    fn encode_f64(value: f64) -> Self::EncodedFp;

    fn encode_vec2(value: DVec2) -> Self::EncodedVec2;
}

impl EncodeShaderFloat for f32 {
    type EncodedFp = f32;
    type EncodedVec2 = Vec2;

    fn encode_f64(value: f64) -> Self::EncodedFp {
        value as f32
    }

    fn encode_vec2(value: DVec2) -> Self::EncodedVec2 {
        vec2(value.x as _, value.y as _)
    }
}

impl EncodeShaderFloat for f64 {
    const IS_DOUBLE_PRECISION: bool = true;

    // we're gonna turn the f64 into bits
    // pack them into a vec2<u32> and in the shader we will
    // merge that into a u64 and bitcast into f64
    type EncodedFp = UVec2;
    type EncodedVec2 = UVec4;

    fn encode_f64(value: f64) -> Self::EncodedFp {
        let bits = value.to_bits();
        let lo = bits as u32;
        let hi = (bits >> 32) as u32;

        uvec2(lo, hi)
    }

    fn encode_vec2(value: DVec2) -> Self::EncodedVec2 {
        let x = Self::encode_f64(value.x);
        let y = Self::encode_f64(value.y);

        uvec4(x.x, x.y, y.x, y.y)
    }
}
