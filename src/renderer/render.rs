use crate::{
    CenterDanmaku,
    Color,
    Danmaku,
    DanmakuMode,
    DanmakuQueue,
    ScrollingDanmaku,
};
use glyphon::{
    Attrs,
    Buffer,
    Cache,
    Family,
    FontSystem,
    Metrics,
    Resolution,
    Shaping,
    SwashCache,
    TextArea,
    TextAtlas,
    TextBounds,
    TextRenderer,
    TextShadow,
    Viewport,
    Weight,
};
use wgpu::{
    CommandEncoderDescriptor,
    LoadOp,
    MultisampleState,
    Operations,
    RenderPassColorAttachment,
    RenderPassDescriptor,
    TextureFormat,
    TextureView,
};

#[cfg(feature = "export-texture")]
use super::export_texture::{
    ExportTexture,
    ExportTextureBuf,
};

pub struct RendererInner {
    pub danmaku_queue: DanmakuQueue,
    pub video_time: f64,
    pub video_speed: f64,

    font_system: FontSystem,
    swash_cache: SwashCache,
    viewport: glyphon::Viewport,
    atlas: glyphon::TextAtlas,
    text_renderer: glyphon::TextRenderer,

    pub paused: bool,

    pub scroll_danmaku: Vec<ScrollingDanmaku>,
    pub scroll_max_rows: usize,

    pub top_center_danmaku: Vec<CenterDanmaku>,
    pub top_center_max_rows: usize,
    pub top_center_row_occupied: Vec<bool>,

    pub bottom_center_danmaku: Vec<CenterDanmaku>,
    pub bottom_center_max_rows: usize,
    pub bottom_center_row_occupied: Vec<bool>,

    pub line_height: f32,
    pub top_padding: f32,
    pub font_size: f32,
    pub font_name: String,
    spacing: f32,
    pub scale_factor: f64,
    pub speed_factor: f64,

    #[cfg(feature = "export-texture")]
    pub texture: Option<ExportTexture>,
    pub texture_view: Option<TextureView>,
    pub shadow: TextShadow,
}

const SCROLL_DURATION_MS: f32 = 8000.0;

impl RendererInner {
    pub fn add_scroll_danmaku(
        &mut self, text_buffer: Buffer, width: f32, text_width: f32, danmaku: Danmaku,
    ) {
        let velocity_x = -(width + text_width) / SCROLL_DURATION_MS * self.speed_factor as f32;

        let v = velocity_x.abs();

        let mut found_row: Option<usize> = None;

        let reach_edge_time = width / v;

        for target_row in 0..self.scroll_max_rows {
            let last_in_row = self
                .scroll_danmaku
                .iter()
                .filter(|d| d.row == target_row)
                .max_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal));

            let Some(last_in_row) = last_in_row else {
                found_row = Some(target_row);
                break;
            };

            let leave_time =
                (last_in_row.x + last_in_row.width + self.spacing) / last_in_row.velocity_x.abs();

            if leave_time < reach_edge_time
                && width > last_in_row.width + self.spacing + last_in_row.x
            {
                found_row = Some(target_row);
                break;
            }
        }

        let Some(target_row) = found_row else {
            return;
        };

        self.scroll_danmaku.push(ScrollingDanmaku {
            danmaku,
            buffer: text_buffer,
            x: width,
            row: target_row,
            velocity_x,
            width: text_width,
        });
    }

    pub fn add_topcenter_danmaku(
        &mut self, text_buffer: Buffer, _width: f32, text_width: f32, danmaku: Danmaku,
    ) {
        let Some(target_row) = self
            .top_center_row_occupied
            .iter()
            .position(|&occupied| !occupied)
        else {
            return;
        };

        self.top_center_row_occupied[target_row] = true;

        self.top_center_danmaku.push(CenterDanmaku {
            danmaku,
            buffer: text_buffer,
            width: text_width,
            row: target_row,
            remaining_time: 5000.0,
        });
    }

    fn add_bottomcenter_danmaku(
        &mut self, text_buffer: Buffer, _width: f32, text_width: f32, danmaku: Danmaku,
    ) {
        let Some(target_row) = self
            .bottom_center_row_occupied
            .iter()
            .position(|&occupied| !occupied)
        else {
            return;
        };

        self.bottom_center_row_occupied[target_row] = true;

        self.bottom_center_danmaku.push(CenterDanmaku {
            danmaku,
            buffer: text_buffer,
            width: text_width,
            row: target_row,
            remaining_time: 5000.0,
        });
    }
}

impl RendererInner {
    pub fn new(
        device: &wgpu::Device, queue: &wgpu::Queue, format: TextureFormat, scale_factor: f64,
    ) -> Self {
        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let cache = Cache::new(device);
        let viewport = Viewport::new(device, &cache);
        let mut atlas = TextAtlas::new(device, queue, &cache, format);
        let text_renderer =
            TextRenderer::new(&mut atlas, device, MultisampleState::default(), None);

        let scroll_max_rows = 20;
        let top_center_max_rows = 10;
        let bottom_center_max_rows = 10;
        let line_height = 30.0 * scale_factor as f32;
        let top_padding = 10.0 * scale_factor as f32;
        let font_size = 24.0 * scale_factor as f32;
        let speed_factor = 1.0;
        let spacing = 20.0 * scale_factor as f32;
        let shadow = TextShadow {
            shadow_intensity: 0.3,
            shadow_radius: 3.0,
        };

        let top_center_row_occupied = vec![false; top_center_max_rows];
        let bottom_center_row_occupied = vec![false; bottom_center_max_rows];

        Self {
            font_name: String::new(),
            danmaku_queue: DanmakuQueue::new(),
            video_time: 0.0,
            video_speed: 1.0,
            font_system,
            swash_cache,
            viewport,
            atlas,
            text_renderer,
            scroll_danmaku: Vec::new(),
            top_center_danmaku: Vec::new(),
            bottom_center_danmaku: Vec::new(),
            scroll_max_rows,
            top_center_max_rows,
            bottom_center_max_rows,
            line_height,
            top_padding,
            font_size,
            scale_factor,
            speed_factor,
            top_center_row_occupied,
            bottom_center_row_occupied,
            paused: false,
            spacing,
            texture_view: None,
            #[cfg(feature = "export-texture")]
            texture: None,
            shadow,
        }
    }

    pub fn add_text(&mut self, danmaku: Danmaku) {
        let font_size = self.font_size;
        let metrics = Metrics::new(font_size, self.line_height);
        let mut text_buffer = Buffer::new(&mut self.font_system, metrics);
        let text_attrs = Attrs::new()
            .family(Family::Name(&self.font_name))
            .weight(Weight::NORMAL);

        text_buffer.set_text(
            &mut self.font_system,
            &danmaku.content,
            &text_attrs,
            Shaping::Advanced,
        );

        let text_width = text_buffer
            .layout_runs()
            .map(|run| run.line_w)
            .reduce(f32::max)
            .unwrap_or(0.0);

        let width = self.viewport.resolution().width as f32;

        match danmaku.mode {
            DanmakuMode::Scroll => {
                self.add_scroll_danmaku(text_buffer, width, text_width, danmaku);
            }
            DanmakuMode::TopCenter => {
                self.add_topcenter_danmaku(text_buffer, width, text_width, danmaku);
            }
            DanmakuMode::BottomCenter => {
                self.add_bottomcenter_danmaku(text_buffer, width, text_width, danmaku);
            }
        }
    }
    pub fn update(&mut self, time_milis: f64) {
        if self.paused {
            return;
        }

        let delta_time = (time_milis - self.video_time) as f32;
        self.video_time = time_milis;

        for next_danmaku in self.danmaku_queue.pop_to_time(self.video_time) {
            self.add_text(next_danmaku);
        }

        for text in self.scroll_danmaku.iter_mut() {
            text.x += text.velocity_x * delta_time * self.speed_factor as f32;
        }

        self.scroll_danmaku.retain(|text| text.x + text.width > 0.0);

        for text in self.top_center_danmaku.iter_mut() {
            text.remaining_time -= delta_time;
        }

        self.top_center_danmaku.retain(|text| {
            if text.remaining_time <= 0.0 {
                if let Some(occupied) = self.top_center_row_occupied.get_mut(text.row) {
                    *occupied = false;
                }
                false
            } else {
                true
            }
        });

        for text in self.bottom_center_danmaku.iter_mut() {
            text.remaining_time -= delta_time;
        }

        self.bottom_center_danmaku.retain(|text| {
            if text.remaining_time <= 0.0 {
                if let Some(occupied) = self.bottom_center_row_occupied.get_mut(text.row) {
                    *occupied = false;
                }
                false
            } else {
                true
            }
        });
    }

    pub fn resize(&mut self, queue: &wgpu::Queue, width: u32, height: u32) {
        self.viewport.update(queue, Resolution { width, height });
    }

    pub fn render(
        &mut self, device: &wgpu::Device, queue: &wgpu::Queue, view: &wgpu::TextureView,
        width: u32, height: u32,
    ) -> Result<(), wgpu::SurfaceError> {
        let scroll_areas = self.scroll_danmaku.iter_mut().map(|text| {
            let top_y = self.top_padding + (text.row as f32 * self.line_height);
            let Color { r, g, b, a } = text.danmaku.color;
            TextArea {
                buffer: &mut text.buffer,
                left: text.x,
                top: top_y,
                scale: 1.0,
                bounds: TextBounds::default(),
                default_color: glyphon::Color::rgba(r, g, b, a),
                custom_glyphs: &[],
                shadow: Some(self.shadow),
            }
        });

        let top_center_areas = self.top_center_danmaku.iter_mut().map(|text| {
            let Color { r, g, b, a } = text.danmaku.color;
            TextArea {
                buffer: &mut text.buffer,
                left: (width as f32 - text.width) / 2.0,
                top: self.top_padding + (text.row as f32 * self.line_height),
                scale: 1.0,
                bounds: TextBounds::default(),
                default_color: glyphon::Color::rgba(r, g, b, a),
                custom_glyphs: &[],
                shadow: Some(self.shadow),
            }
        });

        let bottom_center_areas = self.bottom_center_danmaku.iter_mut().map(|text| {
            let Color { r, g, b, a } = text.danmaku.color;
            TextArea {
                buffer: &mut text.buffer,
                left: (width as f32 - text.width) / 2.0,
                top: height as f32 - self.top_padding - ((text.row + 1) as f32 * self.line_height),
                scale: 1.0,
                bounds: TextBounds::default(),
                default_color: glyphon::Color::rgba(r, g, b, a),
                custom_glyphs: &[],
                shadow: Some(self.shadow),
            }
        });

        let areas = scroll_areas
            .chain(top_center_areas)
            .chain(bottom_center_areas);

        self.text_renderer
            .prepare(
                device,
                queue,
                &mut self.font_system,
                &mut self.atlas,
                &self.viewport,
                areas,
                &mut self.swash_cache,
            )
            .unwrap();

        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.text_renderer
                .render(&self.atlas, &self.viewport, &mut pass)
                .unwrap();
        }

        queue.submit(Some(encoder.finish()));

        device.poll(wgpu::PollType::Wait).unwrap();

        Ok(())
    }
}

#[cfg(feature = "export-texture")]
impl RendererInner {
    pub fn render_to_export_texture(
        &mut self, device: &wgpu::Device, instance: &wgpu::Instance, queue: &wgpu::Queue,
        width: u32, height: u32,
    ) -> Result<ExportTextureBuf, wgpu::SurfaceError> {
        let target_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        if self
            .texture
            .as_ref()
            .is_none_or(|tex| tex.size != target_size)
        {
            let new_texture = ExportTexture::new(device, instance, target_size);
            let new_view = new_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            self.texture = Some(new_texture);
            self.texture_view = Some(new_view);
            self.viewport.update(queue, Resolution { width, height });
        }

        let texture = self.texture.as_ref().unwrap();

        let scroll_areas = self.scroll_danmaku.iter_mut().map(|text| {
            let top_y = self.top_padding + (text.row as f32 * self.line_height);
            let Color { r, g, b, a } = text.danmaku.color;
            TextArea {
                buffer: &mut text.buffer,
                left: text.x,
                top: top_y,
                scale: 1.0,
                bounds: TextBounds::default(),
                default_color: glyphon::Color::rgba(r, g, b, a),
                custom_glyphs: &[],
                shadow: Some(self.shadow),
            }
        });

        let top_center_areas = self.top_center_danmaku.iter_mut().map(|text| {
            let Color { r, g, b, a } = text.danmaku.color;
            TextArea {
                buffer: &mut text.buffer,
                left: (width as f32 - text.width) / 2.0,
                top: self.top_padding + (text.row as f32 * self.line_height),
                scale: 1.0,
                bounds: TextBounds::default(),
                default_color: glyphon::Color::rgba(r, g, b, a),
                custom_glyphs: &[],
                shadow: Some(self.shadow),
            }
        });

        let bottom_center_areas = self.bottom_center_danmaku.iter_mut().map(|text| {
            let Color { r, g, b, a } = text.danmaku.color;
            TextArea {
                buffer: &mut text.buffer,
                left: (width as f32 - text.width) / 2.0,
                top: height as f32 - self.top_padding - ((text.row + 1) as f32 * self.line_height),
                scale: 1.0,
                bounds: TextBounds::default(),
                default_color: glyphon::Color::rgba(r, g, b, a),
                custom_glyphs: &[],
                shadow: Some(self.shadow),
            }
        });

        let areas = scroll_areas
            .chain(top_center_areas)
            .chain(bottom_center_areas);

        self.text_renderer
            .prepare(
                device,
                queue,
                &mut self.font_system,
                &mut self.atlas,
                &self.viewport,
                areas,
                &mut self.swash_cache,
            )
            .unwrap();

        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Danmaku Render Encoder"),
        });

        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Danmaku Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: self.texture_view.as_ref().unwrap(),
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.text_renderer
                .render(&self.atlas, &self.viewport, &mut pass)
                .unwrap();
        }

        queue.submit(Some(encoder.finish()));

        device.poll(wgpu::PollType::Wait).unwrap();

        let texture_buf = ExportTextureBuf {
            fd: texture.fd,
            row_stride: texture.row_stride,
            size: texture.size,
        };

        Ok(texture_buf)
    }
}
