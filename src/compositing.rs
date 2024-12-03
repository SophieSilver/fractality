use bevy::{prelude::*, render::camera::ScalingMode};

/// Plugin responsible for managing different viewports of the app
pub struct CompositingPlugin;

impl Plugin for CompositingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_fractal_camera);
    }
}

/// Camera responsible for compositing the final fractal to the screen
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Component)]
#[require(Camera2d, OrthographicProjection(fractal_camera_projection))]
pub struct FractalCamera;

fn fractal_camera_projection() -> OrthographicProjection {
    OrthographicProjection {
        scale: 2.0,
        scaling_mode: ScalingMode::AutoMax {
            max_width: 1.0,
            max_height: 1.0,
        },
        ..OrthographicProjection::default_2d()
    }
}

pub fn add_fractal_camera(mut commands: Commands) {
    commands.spawn(FractalCamera);
}
