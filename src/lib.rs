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

pub use renderer::Renderer;
