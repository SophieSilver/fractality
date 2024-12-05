//! Module that is responsible to rendering the fractal to an intermediate texture so that
//! It does not need to be rerendered

use crate::compositing::{add_viewport_camera, fractal_camera_projection, ViewportCamera};
use bevy::{
    asset::RenderAssetUsages,
    ecs::{
        query::QuerySingleError,
        schedule::common_conditions::{self, on_event},
    },
    math::uvec2,
    prelude::*,
    render::{
        camera::RenderTarget,
        mesh::{MeshVertexAttribute, PrimitiveTopology, VertexAttributeValues},
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages, VertexFormat},
        view::RenderLayers,
    },
    sprite::AlphaMode2d,
    window::WindowResized,
};
use bevy_image::Image;
use texture_material::{FractalTextureMaterial, FractalTextureMaterialPlugin};

use super::{material::FractalMaterial, Fractal};

pub mod texture_material;

/// The layer the fractal should be draw to
pub const FRACTAL_LAYER: usize = 1;
const FRACTAL_POSITION: u64 = 1419184817364816;

/// Plugin trat renders the fractal to the screen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FractalRenderingPlugin;

impl Plugin for FractalRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FractalTextureMaterialPlugin);
        app.insert_resource(NeedsRerender(true));
        app.add_systems(
            Startup,
            initialize_fractal_rendering.after(add_viewport_camera),
        );

        app.add_systems(
            PostUpdate,
            (
                (
                    resize_fractal_texture.run_if(on_event::<WindowResized>),
                    trigger_render_on_fractal_change,
                )
                    .before(rerender),
                rerender,
            ),
        );
        app.add_systems(PreUpdate, disable_camera_after_render);
    }
}

/// Flag the tells the renderer that the fractal should be rerendered
#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource, Deref, DerefMut)]
pub struct NeedsRerender(pub bool);

/// The intermediate texture the fractal is rendered to
#[derive(Debug, Clone, PartialEq, Eq, Default, Component, Deref, DerefMut)]
#[require(MeshMaterial2d<FractalTextureMaterial>)]
pub struct FractalTexture(pub Handle<Image>);

/// The camera that renders the fractal to the FractalTexture
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
#[require(Camera2d, OrthographicProjection(fractal_camera_projection))]
pub struct FractalRenderCamera;

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

/// Create the texture used for the fractal to the fractal texture
pub fn create_fractal_texture(size: UVec2) -> Image {
    let mut fractal_image = Image::new_fill(
        Extent3d {
            width: size.x,
            height: size.y,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0u8; 4],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::all(),
    );
    fractal_image.texture_descriptor.usage |= TextureUsages::RENDER_ATTACHMENT;
    fractal_image
}

pub fn initialize_fractal_rendering(
    mut commands: Commands,
    mut image_assets: ResMut<Assets<Image>>,
    mut material_assets: ResMut<Assets<FractalTextureMaterial>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    viewport_camera: Query<&Camera, With<ViewportCamera>>,
) {
    let viewport_camera = viewport_camera.single();
    let viewport_size = viewport_camera
        .physical_viewport_size()
        .unwrap_or(uvec2(1280, 720))
        .max(uvec2(1, 1));

    let texture = create_fractal_texture(viewport_size);
    let texture_handle = image_assets.add(texture);

    let material = material_assets.add(FractalTextureMaterial {
        texture: texture_handle.clone(),
    });

    let mesh = mesh_assets.add(create_fractal_mesh());

    commands.spawn((
        FractalTexture(texture_handle.clone()),
        Mesh2d(mesh),
        MeshMaterial2d(material),
    ));
    commands.spawn((
        FractalRenderCamera,
        Camera {
            target: RenderTarget::Image(texture_handle),
            ..default()
        },
        RenderLayers::layer(FRACTAL_LAYER),
    ));
}

pub fn disable_camera_after_render(
    mut camera: Query<&mut Camera, With<FractalRenderCamera>>,
    mut needs_rerender: ResMut<NeedsRerender>,
) {
    if !needs_rerender.0 {
        return;
    }

    camera.single_mut().is_active = false;
    needs_rerender.0 = false;
}

pub fn rerender(
    mut camera: Query<&mut Camera, With<FractalRenderCamera>>,
    needs_rerender: Res<NeedsRerender>,
) {
    if !needs_rerender.0 {
        return;
    }

    camera.single_mut().is_active = true;
}

pub fn trigger_render_on_fractal_change(
    changed: Query<(), Changed<Fractal>>,
    mut needs_rerender: ResMut<NeedsRerender>,
) {
    if changed.is_empty() {
        return;
    }

    needs_rerender.0 = true;
}

pub fn resize_fractal_texture(
    fractal_texture: Query<(&FractalTexture, &MeshMaterial2d<FractalTextureMaterial>)>,
    viewport_camera: Query<&Camera, With<ViewportCamera>>,
    mut render_camera: Query<&mut Camera, (With<FractalRenderCamera>, Without<ViewportCamera>)>,
    mut image_assets: ResMut<Assets<Image>>,
    mut material_assets: ResMut<Assets<FractalTextureMaterial>>,
    mut needs_rerender: ResMut<NeedsRerender>,
) {
    let viewport_camera = match viewport_camera.get_single() {
        Ok(value) => value,
        Err(QuerySingleError::NoEntities(_)) => return,
        Err(e) => panic!("{e}"),
    };

    let Some(new_size) = viewport_camera
        .physical_viewport_size()
        .map(|size| size.max(uvec2(1, 1)))
    else {
        warn!("Can't get physical size");
        return;
    };

    let (fractal_texture, material_handle) = fractal_texture.single();
    let texture_asset = image_assets.get(fractal_texture.id()).unwrap();
    // if texture_asset.size() == new_size {
    //     return;
    // }
    let texture_asset = image_assets.get_mut(fractal_texture.id()).unwrap();
    texture_asset.resize(Extent3d {
        width: new_size.x,
        height: new_size.y,
        depth_or_array_layers: 1,
    });

    // I don't fucking know why the fuck you have to reassign the texture to them
    // but it doesn't fucking work without it
    let material = material_assets.get_mut(material_handle).unwrap();
    material.texture = fractal_texture.0.clone();
    render_camera.single_mut().target = RenderTarget::Image(fractal_texture.0.clone());

    needs_rerender.0 = true;
    debug!(%new_size, "Resized fractal texture");
}
