#![allow(dead_code, unused_imports)]
use bevy::{prelude::*, render::prelude::*};
use fractality::{
    compositing::CompositingPlugin, fps_title::FpsTitlePlugin, fractal::FractalPlugin,
    input::FractalInputPlugin,
};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        FpsTitlePlugin,
        FractalPlugin,
        FractalInputPlugin,
        CompositingPlugin,
    ))
    .add_systems(Startup, initialize);
    app.run();
}

fn initialize(mut msaa: Query<&mut Msaa>, mut _window: Query<&mut Window>) {
    for mut msaa in msaa.iter_mut() {
        *msaa = Msaa::Off;
    }
    // window.get_single_mut().unwrap().present_mode = PresentMode::AutoNoVsync;
}
