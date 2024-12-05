use std::time::Duration;

use crate::{compositing::ViewportCamera, input::FractalInputPlugin};
use bevy::{
    ecs::query::QuerySingleError, prelude::*, render::view::RenderLayers, sprite::Material2dPlugin,
    time::common_conditions,
};
use material::{FractalMaterial, FractalMaterialPlugin};
use render::{create_fractal_mesh, FractalRenderingPlugin, FRACTAL_LAYER};

pub mod material;
pub mod render;

#[derive(Debug, Clone, Copy, Default)]
pub struct FractalPlugin;

impl Plugin for FractalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FractalMaterialPlugin);
        app.add_plugins(FractalRenderingPlugin);
        app.add_systems(Startup, add_fractal_to_world);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub struct Fractal {
    pub scale: f32,
    pub offset: Vec2,
}

impl Default for Fractal {
    fn default() -> Self {
        Self {
            scale: 2.0,
            offset: Vec2::ZERO,
        }
    }
}

pub fn add_fractal_to_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FractalMaterial>>,
) {
    let mesh = meshes.add(create_fractal_mesh());
    let material = materials.add(FractalMaterial::default());

    commands.spawn((
        Fractal::default(),
        Mesh2d(mesh),
        MeshMaterial2d(material),
        RenderLayers::layer(FRACTAL_LAYER),
    ));
}
