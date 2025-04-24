mod gtk;

use danmakw::Renderer;
use gdk::gio::prelude::ApplicationExtManual;
use std::sync::Arc;
use wgpu::{
    CompositeAlphaMode,
    DeviceDescriptor,
    Instance,
    InstanceDescriptor,
    PresentMode,
    RequestAdapterOptions,
    SurfaceConfiguration,
    TextureUsages,
    TextureViewDescriptor,
};

fn main() {
    let instance = Instance::new(&InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        backend_options: wgpu::BackendOptions::from_env_or_default(),
        flags: wgpu::InstanceFlags::default(),
    });

    gtk::new_app().run();
}