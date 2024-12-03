use std::time::Duration;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    time::common_conditions::on_timer,
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

pub fn fps_title_system(mut window: Query<&mut Window>, diagnostics: ResMut<DiagnosticsStore>) {
    let Ok(mut window) = window.get_single_mut() else {
        return;
    };
    let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) else {
        return;
    };

    let Some(average_fps) = fps.average() else {
        return;
    };

    window.title = format!("Fractality ({average_fps:.2} FPS)");
}
