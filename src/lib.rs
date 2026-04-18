mod danmaku;
mod renderer;
mod gtkgl;

pub use gtkgl::*;
pub use danmaku::{
    CenterDanmaku,
    Color,
    Danmaku,
    DanmakuMode,
    DanmakuQueue,
    ScrollingDanmaku,
};
pub use renderer::Renderer;

use gtk::prelude::*;

pub fn init() {
    DanmakwArea::ensure_type();
}
