#![feature(duration_millis_float)]
mod danmaku;
mod renderer;

pub use danmaku::CenterDanmaku;
pub use danmaku::Color;
pub use danmaku::Danmaku;
pub use danmaku::DanmakuMode;
pub use danmaku::ScrollingDanmaku;

pub use renderer::Renderer;
