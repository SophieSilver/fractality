use bevy::{
    prelude::*,
    render::{
        mesh::VertexBufferLayout,
        render_resource::{AsBindGroup, ShaderRef, VertexAttribute, VertexFormat, VertexStepMode},
    },
    sprite::{AlphaMode2d, Material2d, Material2dPlugin},
};
use bevy_image::Image;

const TEXTURE_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0x4509dd70_98fa_4797_84ea_cc1933f1500c);

/// Plugin for the FractalTextureMaterial
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FractalTextureMaterialPlugin;

impl Plugin for FractalTextureMaterialPlugin {
    fn build(&self, app: &mut App) {
        if !cfg!(debug_assertions) {
            let mut shaders = app.world_mut().resource_mut::<Assets<Shader>>();
            shaders.get_or_insert_with(TEXTURE_SHADER_HANDLE.id(), || {
                Shader::from_wgsl(
                    include_str!("../../../assets/shaders/texture.wgsl"),
                    "assets/shaders/texture.wgsl",
                )
            });
        }
        app.add_plugins(Material2dPlugin::<FractalTextureMaterial>::default());
    }
}

/// Material for just drawing the contens of a texture into a fullscreen triangle in front of camera
#[derive(Debug, Clone, PartialEq, Eq, Default, Asset, AsBindGroup, TypePath)]
pub struct FractalTextureMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
}

impl Material2d for FractalTextureMaterial {
    fn vertex_shader() -> ShaderRef {
        if cfg!(debug_assertions) {
            ShaderRef::Path("shaders/texture.wgsl".into())
        } else {
            ShaderRef::Handle(TEXTURE_SHADER_HANDLE)
        }
    }

    fn fragment_shader() -> ShaderRef {
        if cfg!(debug_assertions) {
            ShaderRef::Path("shaders/texture.wgsl".into())
        } else {
            ShaderRef::Handle(TEXTURE_SHADER_HANDLE)
        }
    }

    fn depth_bias(&self) -> f32 {
        0.0
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Opaque
    }

    fn specialize(
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        _layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::sprite::Material2dKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
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
