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

#[derive(Debug, Clone, Copy)]
pub(crate) enum EitherIterator<A, B> {
    A(A),
    B(B),
}

impl<A, B, T> Iterator for EitherIterator<A, B>
where
    A: Iterator<Item = T>,
    B: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            EitherIterator::A(a) => a.next(),
            EitherIterator::B(b) => b.next(),
        }
    }
}
