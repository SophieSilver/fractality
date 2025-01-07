use bevy::{
    render::{
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    },
    utils::default,
};
use wgpu::{Backends, Features};

pub fn get_default_render_plugin() -> RenderPlugin {
    let vulkan_supported = wgpu::Instance::enabled_backend_features().contains(Backends::VULKAN);

    RenderPlugin {
        render_creation: RenderCreation::Automatic(WgpuSettings {
            backends: vulkan_supported.then_some(Backends::VULKAN),
            ..default()
        }),
        ..default()
    }
}
