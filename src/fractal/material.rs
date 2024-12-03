use bevy::{
    asset::RenderAssetUsages,
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
    sprite::{AlphaMode2d, Material2d, Material2dKey},
};

use super::{FRACTAL_POSITION, FRACTAL_SHADER_HANDLE};

pub fn create_fractal_mesh() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        MeshVertexAttribute::new(
            "FRACTAL_POSITION",
            FRACTAL_POSITION,
            VertexFormat::Float32x2,
        ),
        // a tri that will cover the entire viewport
        VertexAttributeValues::Float32x2(vec![[-1.0, -1.0], [-1.0, 3.0], [3.0, -1.0]]),
    )
}

#[derive(Debug, Clone, Copy, Default, Asset, TypePath, AsBindGroup, ShaderType)]
#[uniform(0, FractalMaterial)] // it's its own uniform
pub struct FractalMaterial {
    scale: f32,
    offset: Vec2,
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
        value.clone()
    }
}
