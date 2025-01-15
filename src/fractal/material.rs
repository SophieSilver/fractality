use super::{parameters::ComplexParameter, Fractal};
use crate::fractal::parameters::Parameter;
use bevy::{
    asset::RenderAssetUsages,
    ecs::query::QuerySingleError,
    prelude::*,
    render::{
        mesh::{
            MeshVertexAttribute, MeshVertexBufferLayoutRef, PrimitiveTopology,
            VertexAttributeValues, VertexBufferLayout,
        },
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, ShaderType,
            SpecializedMeshPipelineError, VertexAttribute, VertexFormat, VertexStepMode,
        },
    },
    sprite::{AlphaMode2d, Material2d, Material2dKey, Material2dPlugin},
};
use shader_float::EncodeShaderFloat;

mod shader_float;
#[cfg(debug_assertions)]
mod shader_hot_reload;
#[cfg(debug_assertions)]
use shader_hot_reload::ShaderHotReloadPlugin;

const FRACTAL_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0xca66eb26_69e9_4e00_8760_ba2d0019c452);

const FRACTAL_SHADER_F64_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0xb6eee0d8_4663_4c6c_8e23_db6d30527739);

const Z_R_VALUE_INDEX: u32 = 0;
const Z_I_VALUE_INDEX: u32 = 1;
const C_R_VALUE_INDEX: u32 = 2;
const C_I_VALUE_INDEX: u32 = 3;
const P_R_VALUE_INDEX: u32 = 4;
const P_I_VALUE_INDEX: u32 = 5;
const PIXEL_X_INDEX: u32 = 6;
const PIXEL_Y_INDEX: u32 = 7;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FractalMaterialPlugin;

impl Plugin for FractalMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            Material2dPlugin::<FractalMaterial<f32>>::default(),
            Material2dPlugin::<FractalMaterial<f64>>::default(),
        ));
        app.add_systems(
            PostUpdate,
            (
                update_fractal_material::<f32>,
                update_fractal_material::<f64>,
            ),
        );

        #[cfg(debug_assertions)]
        app.add_plugins(ShaderHotReloadPlugin);
        
        #[cfg(not(debug_assertions))]
        {
            use bevy::render::render_resource::ShaderDefVal;

            let mut shaders = app.world_mut().resource_mut::<Assets<Shader>>();
            shaders.insert(
                &FRACTAL_SHADER_HANDLE,
                Shader::from_wgsl(
                    include_str!("../../assets/shaders/fractal.wgsl"),
                    "shaders/fractal.wgsl",
                ),
            );
            shaders.insert(
                &FRACTAL_SHADER_F64_HANDLE,
                Shader::from_wgsl_with_defs(
                    include_str!("../../assets/shaders/fractal.wgsl"),
                    "shaders/fractal.wgsl",
                    vec![ShaderDefVal::Bool("DOUBLE_PRECISION".into(), true)],
                ),
            );
        }
    }
}

pub fn create_fractal_mesh() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        MeshVertexAttribute::new("FRACTAL_POSITION", 0, VertexFormat::Float32x2),
        // a tri that will cover the entire viewport
        VertexAttributeValues::Float32x2(vec![[-1.0, -1.0], [-1.0, 3.0], [3.0, -1.0]]),
    )
}

#[derive(Debug, Clone, Copy, Asset, TypePath, AsBindGroup, ShaderType)]
pub struct FractalMaterial<FP: EncodeShaderFloat> {
    #[uniform(0)]
    uniform: MaterialUniform<FP>,
}

#[derive(Debug, Clone, Copy, ShaderType)]
struct MaterialUniform<FP: EncodeShaderFloat> {
    iteration_count: u32,
    scale: FP::EncodedFp,
    offset: FP::EncodedVec2,
    initial_z: EncodedComplexParameter<FP>,
    c: EncodedComplexParameter<FP>,
    p: EncodedComplexParameter<FP>,
    escape_radius: FP::EncodedFp,
}

impl<FP: EncodeShaderFloat> Default for FractalMaterial<FP> {
    fn default() -> Self {
        Fractal::default().into()
    }
}

impl<FP: EncodeShaderFloat + Clone> Material2d for FractalMaterial<FP> {
    fn vertex_shader() -> ShaderRef {
        if FP::IS_DOUBLE_PRECISION {
            ShaderRef::Handle(FRACTAL_SHADER_F64_HANDLE)
        } else {
            ShaderRef::Handle(FRACTAL_SHADER_HANDLE)
        }
    }

    fn fragment_shader() -> ShaderRef {
        if FP::IS_DOUBLE_PRECISION {
            ShaderRef::Handle(FRACTAL_SHADER_F64_HANDLE)
        } else {
            ShaderRef::Handle(FRACTAL_SHADER_HANDLE)
        }
    }

    fn depth_bias(&self) -> f32 {
        0.0
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Opaque
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_buf_layout = &mut descriptor.vertex.buffers;
        vertex_buf_layout.clear();
        vertex_buf_layout.push(VertexBufferLayout {
            array_stride: VertexFormat::Float32x2.size(),
            step_mode: VertexStepMode::Vertex,
            attributes: vec![VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: 0,
                shader_location: 0,
            }],
        });

        Ok(())
    }
}

// This is needed for shader type derives
impl<FP: EncodeShaderFloat> From<&FractalMaterial<FP>> for FractalMaterial<FP> {
    fn from(value: &FractalMaterial<FP>) -> Self {
        *value
    }
}

impl<FP: EncodeShaderFloat> From<Fractal> for FractalMaterial<FP> {
    fn from(fractal: Fractal) -> Self {
        Self {
            uniform: MaterialUniform {
                iteration_count: fractal.iteration_count,
                scale: FP::encode_f64(fractal.scale),
                escape_radius: FP::encode_f64(fractal.escape_radius),
                offset: FP::encode_vec2(fractal.offset),
                initial_z: encode_complex_parameter(
                    fractal.initial_z,
                    Z_R_VALUE_INDEX,
                    Z_I_VALUE_INDEX,
                ),
                c: encode_complex_parameter(fractal.c, C_R_VALUE_INDEX, C_I_VALUE_INDEX),
                p: encode_complex_parameter(fractal.p, P_R_VALUE_INDEX, P_I_VALUE_INDEX),
            },
        }
    }
}

pub fn update_fractal_material<FP: EncodeShaderFloat>(
    query: Query<(&Fractal, &MeshMaterial2d<FractalMaterial<FP>>), Changed<Fractal>>,
    mut materials: ResMut<Assets<FractalMaterial<FP>>>,
) {
    let (fractal, material) = match query.get_single() {
        Ok(value) => value,
        Err(QuerySingleError::NoEntities(_)) => return,
        Err(e) => panic!("{e}"),
    };
    debug!(?fractal, "Updating fractal");

    let Some(material) = materials.get_mut(material.0.id()) else {
        warn!("Failed to find the fractal material asset");
        return;
    };

    *material = (*fractal).into();
}

#[derive(Debug, Clone, Copy, ShaderType)]
struct EncodedComplexParameter<FP: EncodeShaderFloat> {
    real_value: FP::EncodedFp,
    real_index: u32,
    imag_value: FP::EncodedFp,
    imag_index: u32,
}

#[derive(Debug, Clone, Copy, ShaderType)]
struct EncodedParameter<FP: EncodeShaderFloat> {
    value: FP::EncodedFp,
    index: u32,
}

fn encode_complex_parameter<FP: EncodeShaderFloat>(
    param: ComplexParameter,
    real_index: u32,
    imag_index: u32,
) -> EncodedComplexParameter<FP> {
    let real = encode_parameter::<FP>(param.real, real_index);
    let imaginary = encode_parameter::<FP>(param.imaginary, imag_index);
    EncodedComplexParameter {
        real_value: real.value,
        real_index: real.index,
        imag_value: imaginary.value,
        imag_index: imaginary.index,
    }
}

fn encode_parameter<FP: EncodeShaderFloat>(
    param: Parameter,
    value_index: u32,
) -> EncodedParameter<FP> {
    match param {
        Parameter::Value(c) => EncodedParameter {
            value: FP::encode_f64(c),
            index: value_index,
        },
        Parameter::PixelX => EncodedParameter {
            value: FP::encode_f64(0.0),
            index: PIXEL_X_INDEX,
        },
        Parameter::PixelY => EncodedParameter {
            value: FP::encode_f64(0.0),
            index: PIXEL_Y_INDEX,
        },
    }
}
