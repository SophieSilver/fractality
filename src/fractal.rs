use bevy::{math::DVec2, prelude::*};
use material::{create_fractal_mesh, FractalMaterial, FractalMaterialPlugin};
use parameters::{ComplexParameter, Parameter};

pub mod material;
pub mod parameters;

#[derive(Debug, Clone, Copy, Default)]
pub struct FractalPlugin;

impl Plugin for FractalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FractalMaterialPlugin);
        app.add_systems(Startup, add_fractal_to_world);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub struct Fractal {
    pub iteration_count: u32,
    pub scale: f64,
    pub escape_radius: f64,
    pub offset: DVec2,
    pub initial_z: ComplexParameter,
    pub c: ComplexParameter,
    pub p: ComplexParameter,
}

impl Default for Fractal {
    fn default() -> Self {
        Self {
            iteration_count: 100,
            scale: 2.0,
            escape_radius: 2.0,
            offset: DVec2::ZERO,
            initial_z: default(),
            c: ComplexParameter {
                real: Parameter::PixelX,
                imaginary: Parameter::PixelY,
            },
            p: ComplexParameter {
                real: Parameter::Value(2.0),
                imaginary: Parameter::Value(0.0),
            },
        }
    }
}

pub fn add_fractal_to_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FractalMaterial<f32>>>,
) {
    let mesh = meshes.add(create_fractal_mesh());
    let material = materials.add(FractalMaterial::default());

    commands.spawn((Fractal::default(), Mesh2d(mesh), MeshMaterial2d(material)));
}
