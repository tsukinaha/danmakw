mod render;
mod export_texture;
pub use export_texture::ExportTexture;
use render::RendererInner;
use wgpu::TextureFormat;

use crate::Danmaku;

pub struct Renderer(pub RendererInner);

impl Renderer {
    pub fn new(
        device: &wgpu::Device, queue: &wgpu::Queue, format: TextureFormat, scale_factor: f64,
    ) -> Self {
        Self(RendererInner::new(device, queue, format, scale_factor))
    }

    pub fn resize(&mut self, queue: &wgpu::Queue, width: u32, height: u32) {
        self.0.resize(queue, width, height);
    }

    // Hard set the video time, Danmaku not presented will be removed
    pub fn set_video_time(&mut self, time: f64) {
        self.0.video_time = time;

        self.0.danmaku_queue.pop_to_time(time);
    }

    pub fn init(&mut self, danmaku: Vec<Danmaku>) {
        self.0.danmaku_queue.init(danmaku, 0.0);
    }

    pub fn update(&mut self) {
        self.0.update();
    }

    pub fn add_text(&mut self, danmaku: Danmaku) {
        self.0.add_text(danmaku);
    }

    pub fn render(
        &mut self, device: &wgpu::Device, queue: &wgpu::Queue, view: &wgpu::TextureView,
        width: u32, height: u32,
    ) -> Result<(), wgpu::SurfaceError> {
        self.0.render(device, queue, view, width, height)
    }

    pub fn render_to_export_texture(
        &mut self, device: &wgpu::Device, instance: &wgpu::Instance, queue: &wgpu::Queue, width: u32, height: u32,
    ) -> Result<ExportTexture, wgpu::SurfaceError> {
        self.0.render_to_export_texture(device, instance, queue, width, height)
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.0.paused = paused;
    }

    pub fn clear(&mut self) {
        self.0.scroll_danmaku.clear();
        self.0.top_center_danmaku.clear();
        self.0.bottom_center_danmaku.clear();
        self.0.top_center_row_occupied.fill(false);
        self.0.bottom_center_row_occupied.fill(false);
    }

    pub fn set_speed_factor(&mut self, speed_factor: f64) {
        self.0.speed_factor = speed_factor;
    }

    pub fn set_font_size(&mut self, font_size: f32) {
        self.0.font_size = font_size;
    }

    pub fn set_line_height(&mut self, line_height: f32) {
        self.0.line_height = line_height;
    }

    pub fn set_top_padding(&mut self, top_padding: f32) {
        self.0.top_padding = top_padding;
    }

    pub fn set_max_rows(&mut self, max_rows: usize) {
        self.0.scroll_max_rows = max_rows;
    }
}
