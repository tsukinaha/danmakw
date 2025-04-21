use std::time::Instant;

use glyphon::Buffer;

#[derive(Debug, Clone, PartialEq)]
pub struct Danmaku<'a> {
    pub content: &'a str,
    pub start: f32,
    pub color: Color,
    pub mode: DanmakuMode,
}

pub struct ScrollingDanmaku<'a> {
    pub danmaku: Danmaku<'a>,
    pub buffer: Buffer,
    pub x: f32,
    pub row: usize,
    pub velocity_x: f32,
    pub width: f32,
}

pub struct CenterDanmaku<'a> {
    pub danmaku: Danmaku<'a>,
    pub buffer: Buffer,
    pub width: f32,
    pub row: usize,
    pub start_time: Instant,
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

pub const TEST_DANMAKU_MAP: [Danmaku; 10] = [
    Danmaku {
        content: "我要退出CRYCHIC",
        start: 0.0,
        color: Color {
            r: 243,
            g: 232,
            b: 122,
            a: 108,
        },
        mode: DanmakuMode::Scroll,
    },
    Danmaku {
        content: "睦酱也是这么觉得的吧",
        start: 1.5,
        color: Color {
            r: 108,
            g: 0,
            b: 0,
            a: 108,
        },
        mode: DanmakuMode::TopCenter,
    },
    Danmaku {
        content: "那……能陪我组一辈子的乐队吗？ 🐢🐢🐢",
        start: 3.0,
        color: Color {
            r: 0,
            g: 108,
            b: 0,
            a: 108,
        },
        mode: DanmakuMode::BottomCenter,
    },
    Danmaku {
        content: "我什么都愿意做",
        start: 4.2,
        color: Color {
            r: 0,
            g: 0,
            b: 108,
            a: 108,
        },
        mode: DanmakuMode::Scroll,
    },
    Danmaku {
        content: "这家伙根本什么也不懂，我要拉黑他",
        start: 5.0,
        color: Color {
            r: 108,
            g: 108,
            b: 108,
            a: 108,
        },
        mode: DanmakuMode::Scroll,
    },
    Danmaku {
        content: "拜托了，初华 让我……忘记一切吧",
        start: 10.0,
        color: Color {
            r: 128,
            g: 128,
            b: 128,
            a: 108,
        },
        mode: DanmakuMode::Scroll,
    },
    Danmaku {
        content: "你是抱着多大的觉悟说出这种话的",
        start: 11.0,
        color: Color {
            r: 108,
            g: 165,
            b: 0,
            a: 108,
        },
        mode: DanmakuMode::TopCenter,
    },
    Danmaku {
        content: "你这个人，满脑子都只想着自己呢",
        start: 12.5,
        color: Color {
            r: 128,
            g: 0,
            b: 128,
            a: 108,
        },
        mode: DanmakuMode::BottomCenter,
    },
    Danmaku {
        content: "α β γ δ ε ζ η θ ι κ λ μ ν ξ ο π ρ σ τ υ φ χ ψ ω",
        start: 15.0,
        color: Color {
            r: 0,
            g: 108,
            b: 108,
            a: 108,
        },
        mode: DanmakuMode::Scroll,
    },
    Danmaku {
        content: "もう関係ないから",
        start: 18.0,
        color: Color {
            r: 108,
            g: 192,
            b: 203,
            a: 108,
        },
        mode: DanmakuMode::Scroll,
    },
];
