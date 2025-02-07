use bevy::{
    ecs::schedule::common_conditions::on_event,
    math::uvec2,
    prelude::*,
    render::camera::{camera_system, ScalingMode, Viewport},
    window::{PrimaryWindow, WindowResized},
};

use crate::ui::NonUiArea;

/// Plugin responsible for managing different viewports of the app
pub struct CompositingPlugin;

impl Plugin for CompositingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_viewport_camera).add_systems(
            PostUpdate,
            (
                deactivate_camera_on_minimize.run_if(on_event::<WindowResized>),
                resize_viewport.run_if(resource_changed::<NonUiArea>.or(on_event::<WindowResized>)),
            )
                .chain()
                .before(camera_system::<OrthographicProjection>),
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
        // should be at least (1, 1)
        physical_size: area.size().max(uvec2(1, 1)),
        depth: 0.0..1.0,
    })
}

// Fixes https://github.com/SophieSilver/fractality/issues/1
pub fn deactivate_camera_on_minimize(
    mut camera: Query<&mut Camera, With<ViewportCamera>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window.get_single() else {
        return;
    };
    let mut camera = camera.single_mut();

    let resolution = window.resolution.physical_size();
    let minimized = resolution == uvec2(0, 0);

    // checking is_active before setting it avoids tripping change detection
    if camera.is_active && minimized {
        debug!("ViewportCamera deactivated");
        camera.is_active = false;
    } else if !camera.is_active && !minimized {
        debug!("ViewportCamera activated");
        camera.is_active = true;
    }
}
