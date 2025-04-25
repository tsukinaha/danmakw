mod gtk;
mod utils;

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

pub use utils::parse_bilibili_xml;

fn main() {
    gtk::new_app().run();
}