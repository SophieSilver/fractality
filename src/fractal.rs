use bevy::{math::DVec2, prelude::*};

pub mod material;
pub mod parameters;
pub mod render;

use material::FractalMaterialPlugin;
use parameters::{ComplexParameter, Parameter};
use render::{FractalRenderPlugin, FractalRenderer};

#[derive(Debug, Clone, Copy, Default)]
pub struct FractalPlugin;

impl Plugin for FractalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FractalMaterialPlugin, FractalRenderPlugin));
        app.add_systems(Startup, add_fractal_to_world);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Component)]
#[require(FractalRenderer)]
pub struct Fractal {
    pub iteration_count: u32,
    pub scale: f64,
    pub escape_radius: f64,
    pub offset: DVec2,
    pub initial_z: ComplexParameter,
    pub c: ComplexParameter,
    pub p: ComplexParameter,
    pub use_f64: bool,
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
            use_f64: false,
        }
    }
}

pub fn add_fractal_to_world(mut commands: Commands) {
    commands.spawn(Fractal::default());
}
