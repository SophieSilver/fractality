use bevy::{
    prelude::*,
    sprite::Material2dPlugin,
};
use material::{create_fractal_mesh, FractalMaterial};

pub mod material;

const FRACTAL_POSITION: u64 = 1419184817364816;

const FRACTAL_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0xca66eb26_69e9_4e00_8760_ba2d0019c452);

#[derive(Debug, Clone, Copy, Default)]
pub struct FractalPlugin;

impl Plugin for FractalPlugin {
    fn build(&self, app: &mut App) {
        // embed the shader in the release executable
        if !cfg!(debug_assertions) {
            let mut shaders = app.world_mut().resource_mut::<Assets<Shader>>();
            shaders.get_or_insert_with(FRACTAL_SHADER_HANDLE.id(), || {
                Shader::from_wgsl(
                    include_str!("../assets/shaders/fractal.wgsl"),
                    "assets/shaders/fractal.wgsl",
                )
            });
        }

        app.add_plugins(Material2dPlugin::<FractalMaterial>::default());
        app.add_systems(Startup, add_fractal_to_world);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct Fractal;

pub fn add_fractal_to_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FractalMaterial>>,
) {
    let mesh = meshes.add(create_fractal_mesh());
    let material = materials.add(FractalMaterial::default());

    commands.spawn((Fractal, Mesh2d(mesh), MeshMaterial2d(material)));
}


