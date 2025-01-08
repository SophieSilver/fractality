use bevy::{
    prelude::*,
    render::camera::{camera_system, ScalingMode, Viewport},
};

use crate::ui::NonUiArea;

/// Plugin responsible for managing different viewports of the app
pub struct CompositingPlugin;

impl Plugin for CompositingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_viewport_camera).add_systems(
            PostUpdate,
            resize_viewport
                .before(camera_system::<OrthographicProjection>)
                .run_if(resource_changed::<NonUiArea>),
        );
    }
}

/// Camera responsible for compositing the final image of the fractal to the screen
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Component)]
#[require(Camera2d, OrthographicProjection(fractal_camera_projection))]
pub struct ViewportCamera;

pub fn fractal_camera_projection() -> OrthographicProjection {
    OrthographicProjection {
        scale: 2.0,
        scaling_mode: ScalingMode::AutoMax {
            max_width: 1.0,
            max_height: 1.0,
        },
        ..OrthographicProjection::default_2d()
    }
}

pub fn add_viewport_camera(mut commands: Commands) {
    commands.spawn(ViewportCamera);
}

pub fn resize_viewport(mut camera: Query<&mut Camera, With<ViewportCamera>>, area: Res<NonUiArea>) {
    debug!(area = ?area.0, "Resizing viewport");
    let mut camera = camera.single_mut();

    camera.viewport = Some(Viewport {
        physical_position: area.min,
        physical_size: area.size(),
        depth: 0.0..1.0,
    })
}
