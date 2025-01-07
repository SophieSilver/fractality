#![allow(dead_code, unused_imports)]
use bevy::{prelude::*, render::prelude::*};
use fractality::{
    compositing::CompositingPlugin, fps_title::FpsTitlePlugin, fractal::FractalPlugin,
    input::FractalInputPlugin, ui::UiPlugin,
};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        FpsTitlePlugin,
        FractalPlugin,
        FractalInputPlugin,
        CompositingPlugin,
        UiPlugin,
    ))
    .add_systems(Startup, on_start);
    app.run();
}

fn on_start(
    mut msaa: Query<&mut Msaa>,
    // mut _window: Query<&mut Window>,
    device: Res<RenderDevice>,
) {
    for mut msaa in msaa.iter_mut() {
        *msaa = Msaa::Off;
    }

    info!(device_features=?device.features(), "");
    // window.get_single_mut().unwrap().present_mode = PresentMode::AutoNoVsync;
}
