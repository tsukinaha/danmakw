mod queue;
mod sort;

pub use queue::DanmakuQueue;

use glyphon::Buffer;

#[derive(Debug, Clone, PartialEq)]
pub struct Danmaku {
    pub content: String,
    // milliseconds
    pub start: f64,
    pub color: Color,
    pub mode: DanmakuMode,
}

pub struct ScrollingDanmaku {
    pub danmaku: Danmaku,
    pub buffer: Buffer,
    pub x: f32,
    pub row: usize,
    pub velocity_x: f32,
    pub width: f32,
}

pub struct CenterDanmaku {
    pub danmaku: Danmaku,
    pub buffer: Buffer,
    pub width: f32,
    pub row: usize,
    pub remaining_time: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DanmakuMode {
    Scroll,
    TopCenter,
    BottomCenter,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
