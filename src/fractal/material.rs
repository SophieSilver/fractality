use bevy::{
    asset::RenderAssetUsages,
    ecs::query::QuerySingleError,
    math::{ivec2, uvec2},
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

use crate::fractal::parameters::Parameter;

use super::{parameters::ComplexParameter, Fractal};

const FRACTAL_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0xca66eb26_69e9_4e00_8760_ba2d0019c452);

const Z_R_VALUE_INDEX: u32 = 0;
const Z_I_VALUE_INDEX: u32 = 1;
const PIXEL_X_INDEX: u32 = 2;
const PIXEL_Y_INDEX: u32 = 3;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FractalMaterialPlugin;

impl Plugin for FractalMaterialPlugin {
    fn build(&self, app: &mut App) {
        if !cfg!(debug_assertions) {
            let mut shaders = app.world_mut().resource_mut::<Assets<Shader>>();
            shaders.get_or_insert_with(FRACTAL_SHADER_HANDLE.id(), || {
                Shader::from_wgsl(
                    include_str!("../../assets/shaders/fractal.wgsl"),
                    "assets/shaders/fractal.wgsl",
                )
            });
        }
        app.add_plugins(Material2dPlugin::<FractalMaterial>::default());
        app.add_systems(PostUpdate, update_fractal_material);
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
#[uniform(0, FractalMaterial)] // it's its own uniform
pub struct FractalMaterial {
    pub scale: f32,
    pub offset: Vec2,
    pub initial_z_values: Vec2,
    pub initial_z_indices: UVec2,
}

impl Default for FractalMaterial {
    fn default() -> Self {
        Self {
            scale: 2.0,
            offset: default(),
            initial_z_values: default(),
            initial_z_indices: uvec2(Z_R_VALUE_INDEX, Z_I_VALUE_INDEX),
        }
    }
}

impl Material2d for FractalMaterial {
    fn vertex_shader() -> ShaderRef {
        if cfg!(debug_assertions) {
            ShaderRef::Path("shaders/fractal.wgsl".into())
        } else {
            ShaderRef::Handle(FRACTAL_SHADER_HANDLE)
        }
    }

    fn fragment_shader() -> ShaderRef {
        if cfg!(debug_assertions) {
            ShaderRef::Path("shaders/fractal.wgsl".into())
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

impl From<&FractalMaterial> for FractalMaterial {
    fn from(value: &FractalMaterial) -> Self {
        *value
    }
}

pub fn update_fractal_material(
    query: Query<(&Fractal, &MeshMaterial2d<FractalMaterial>), Changed<Fractal>>,
    mut materials: ResMut<Assets<FractalMaterial>>,
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

    material.scale = fractal.scale;
    material.offset = fractal.offset;
    load_complex_param(
        fractal.initial_z,
        Z_R_VALUE_INDEX,
        Z_I_VALUE_INDEX,
        &mut material.initial_z_values,
        &mut material.initial_z_indices,
    );
}

fn load_complex_param(
    param: ComplexParameter,
    real_index: u32,
    imag_index: u32,
    value_target: &mut Vec2,
    index_target: &mut UVec2,
) {
    load_parameter(
        param.real,
        real_index,
        &mut value_target.x,
        &mut index_target.x,
    );

    load_parameter(
        param.imaginary,
        imag_index,
        &mut value_target.y,
        &mut index_target.y,
    );
}

fn load_parameter(
    param: Parameter,
    value_index: u32,
    value_target: &mut f32,
    index_target: &mut u32,
) {
    match param {
        Parameter::Value(c) => {
            *value_target = c;
            *index_target = value_index;
        }
        Parameter::PixelX => *index_target = PIXEL_X_INDEX,
        Parameter::PixelY => *index_target = PIXEL_Y_INDEX,
    }
}
