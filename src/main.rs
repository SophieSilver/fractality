#![windows_subsystem = "windows"]

// #![allow(dead_code, unused_imports)]
use bevy::{prelude::*, render::renderer::RenderDevice};
use fractality::{
    compositing::CompositingPlugin, fps_title::FpsTitlePlugin, fractal::FractalPlugin,
    input::FractalInputPlugin, panic_hook::PanicHookPlugin, ui::UiPlugin,
    utils::get_default_render_plugin,
};

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.build().set(get_default_render_plugin()),
        PanicHookPlugin,
        FpsTitlePlugin,
        FractalPlugin,
        FractalInputPlugin,
        CompositingPlugin,
        UiPlugin,
    ))
    .add_systems(Startup, on_start);

    app.run()
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
