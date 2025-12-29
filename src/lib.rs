mod danmaku;
mod renderer;

pub use danmaku::{
    CenterDanmaku,
    Color,
    Danmaku,
    DanmakuMode,
    DanmakuQueue,
    ScrollingDanmaku,
};

#[cfg(feature = "export-texture")]
pub use renderer::ExportTexture;

#[cfg(feature = "export-texture")]
pub use renderer::ExportTextureBuf;

pub use renderer::Renderer;

#[cfg(feature = "gtk4-gles")]
mod gtk_wgpu_gles_framebuffer_example;
#[cfg(feature = "gtk4-gles")]
use gdk::glib::types::StaticTypeExt;
#[cfg(feature = "gtk4-gles")]
pub use gtk_wgpu_gles_framebuffer_example::*;
#[cfg(feature = "gtk4-gles")]
pub fn init() {
    DanmakwArea::ensure_type();
}
