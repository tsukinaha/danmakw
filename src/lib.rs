#![feature(duration_millis_float)]
mod danmaku;
mod renderer;

pub use danmaku::{
    CenterDanmaku,
    Color,
    Danmaku,
    DanmakuMode,
    ScrollingDanmaku,
};

pub use renderer::{
    ExportTexture,
    ExportTextureBuf,
    Renderer,
};

#[cfg(feature = "gtk4-gles")]
mod gtk_wgpu_gles_framebuffer_example;
#[cfg(feature = "gtk4-gles")]
pub use gtk_wgpu_gles_framebuffer_example::DanmakwArea;
