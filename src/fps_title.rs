use std::time::Duration;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    time::common_conditions::on_timer,
    window::PrimaryWindow,
};

pub struct FpsTitlePlugin;

impl Plugin for FpsTitlePlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<FrameTimeDiagnosticsPlugin>() {
            app.add_plugins(FrameTimeDiagnosticsPlugin);
        }

        app.add_systems(
            PostUpdate,
            fps_title_system.run_if(on_timer(Duration::from_secs_f32(0.5))),
        );
    }
}

pub fn fps_title_system(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    diagnostics: ResMut<DiagnosticsStore>,
) {
    let Ok(mut window) = window.get_single_mut() else {
        return;
    };

    let d = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .unwrap();
    let Some(avg_time) = d.average() else {
        return;
    };
    let fps = 1.0 / (avg_time / 1000.0);

    window.title = format!("Fractality ({fps:.2} FPS)");
}
