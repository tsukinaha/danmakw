mod danmaku;
mod renderer;
mod gtkgl;
mod clock;

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
pub use clock::DanmakuClock;

use gtk::prelude::*;

pub fn init() {
    DanmakwArea::ensure_type();
}
