use gtk::{
    prelude::*,
    subclass::prelude::*,
};
use std::num::NonZeroU32;
use wgpu::{
    InstanceDescriptor,
    TextureUsages,
};
use wgpu_hal::{
    self as hal,
    Adapter,
    api,
    gles::TextureInner,
};
use wgpu_types::{
    self as wgt,
};

pub struct Renderer {
    fbo: glow::NativeFramebuffer,
    pub instance: wgpu::Instance,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub danmaku_renderer: danmakw::Renderer,
}

impl Renderer {
    pub async fn new() -> Self {
        use glow::HasContext;

        static LOAD_FN: fn(&str) -> *const std::ffi::c_void =
            |s| epoxy::get_proc_addr(s) as *const _;

        let instance = wgpu::Instance::new(&InstanceDescriptor {
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let exposed = unsafe {
            wgpu_hal::gles::Adapter::new_external(
                LOAD_FN,
                wgpu::GlBackendOptions::from_env_or_default(),
            )
        }
        .expect("Initializing new wgpu_hal gles Adapter.");

        let od = unsafe {
            exposed.adapter.open(
                wgt::Features::empty(),
                &wgt::Limits::downlevel_defaults(),
                &wgt::MemoryHints::default(),
            )
        }
        .unwrap();

        let adapter = unsafe { instance.create_adapter_from_hal(exposed) };

        // also need the nativeframebuffer here
        let fbo = unsafe {
            let ctx = glow::Context::from_loader_function(LOAD_FN);
            let id = NonZeroU32::new(ctx.get_parameter_i32(glow::DRAW_FRAMEBUFFER_BINDING) as u32)
                .expect("No GTK provided framebuffer binding");
            ctx.bind_framebuffer(glow::FRAMEBUFFER, None);
            // the view will be created by glow after binding to the correct framebuffer;
            glow::NativeFramebuffer(id)
        };

        let (device, queue) = unsafe {
            adapter
                .create_device_from_hal(od, &Default::default())
                .expect("Failed to create wgpu device from HAL device")
        };

        let danmaku_renderer =
            danmakw::Renderer::new(&device, &queue, wgpu::TextureFormat::Rgba8UnormSrgb, 1.0);

        let (texture, view) = Self::create_texture_and_view(100, 100, fbo, &device);

        Self {
            fbo,
            instance,
            device,
            queue,
            danmaku_renderer,
            texture,
            view,
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        let (texture, view) = Self::create_texture_and_view(width, height, self.fbo, &self.device);

        self.texture = texture;
        self.view = view;

        self.danmaku_renderer.resize(&self.queue, width, height);
    }

    fn create_texture_and_view(
        width: u32, height: u32, fbo: glow::NativeFramebuffer, device: &wgpu::Device,
    ) -> (wgpu::Texture, wgpu::TextureView) {
        let mut texture_hal = <hal::api::Gles as hal::Api>::Texture::default_framebuffer(
            wgt::TextureFormat::Rgba8UnormSrgb,
        );

        texture_hal.inner = TextureInner::ExternalGlFrameBuffer { inner: fbo };

        let texture = unsafe {
            device.create_texture_from_hal::<api::Gles>(
                texture_hal,
                &wgpu_types::TextureDescriptor {
                    label: None,
                    size: wgt::Extent3d {
                        width,
                        height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgt::TextureDimension::D2,
                    format: wgt::TextureFormat::Rgba8UnormSrgb,
                    usage: TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                },
            )
        };

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        (texture, view)
    }

    pub fn render(&mut self, width: u32, height: u32) {
        self.danmaku_renderer.update();

        self.resize(width, height);

        self.danmaku_renderer
            .render(&self.device, &self.queue, &self.view, width, height)
            .unwrap();
    }

    pub fn init(&mut self, danmaku: Vec<danmakw::Danmaku>) {
        self.danmaku_renderer.init(danmaku);
    }
}
