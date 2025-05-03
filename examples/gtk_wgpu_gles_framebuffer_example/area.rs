use glow;
use gtk::{
    Align,
    gdk,
    glib,
    prelude::*,
    subclass::prelude::*,
};
use std::{
    cell::RefCell,
    num::NonZeroU32,
};
use wgpu::{
    InstanceDescriptor,
    TextureUsages,
    TextureUses,
};
use wgpu_hal::{
    self as hal,
    Adapter,
    Device,
    api,
    gles::TextureInner,
};
use wgpu_types::{
    self as wgt,
    DeviceDescriptor,
};

struct Renderer {
    fbo: glow::NativeFramebuffer,
    pub instance: wgpu::Instance,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub danmaku_renderer: danmakw::Renderer,
}

impl Renderer {
    async fn new() -> Self {
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

    fn render(&mut self, width: u32, height: u32) {
        self.danmaku_renderer.update();

        self.resize(width, height);

        println!("Rendering to texture");

        self.danmaku_renderer
            .render(&self.device, &self.queue, &self.view, width, height)
            .unwrap();
    }

    pub fn init(&mut self, danmaku: Vec<danmakw::Danmaku>) {
        self.danmaku_renderer.init(danmaku);
    }
}

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct WgpuArea {
        renderer: RefCell<Option<Renderer>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WgpuArea {
        const NAME: &'static str = "WgpuArea";
        type Type = super::WgpuArea;
        type ParentType = gtk::GLArea;
    }

    impl ObjectImpl for WgpuArea {
        fn constructed(&self) {
            self.parent_constructed();

            self.obj().set_has_stencil_buffer(true);
            self.obj().set_has_depth_buffer(true);

            self.obj().set_halign(Align::Fill);
            self.obj().set_valign(Align::Fill);
            self.obj().set_hexpand(true);
            self.obj().set_vexpand(true);

            glib::spawn_future_local(glib::clone!(
                #[weak(rename_to = imp)]
                self,
                async move {
                    if imp.renderer.borrow().is_some() {
                        return;
                    }
                    imp.obj().attach_buffers();
                    let mut renderer = Renderer::new().await;
                    let danmakus =
                        crate::utils::parse_bilibili_xml(include_str!("../test.xml")).unwrap();
                    renderer.init(danmakus);
                    renderer.danmaku_renderer.set_font_name(imp.font_name());
                    imp.renderer.replace(Some(renderer));
                }
            ));

            self.obj().add_tick_callback(glib::clone!(
                #[weak(rename_to = imp)]
                self,
                #[upgrade_or]
                glib::ControlFlow::Continue,
                move |_, _| {
                    imp.obj().queue_draw();
                    glib::ControlFlow::Continue
                }
            ));
        }
    }

    impl WidgetImpl for WgpuArea {
        fn realize(&self) {
            self.parent_realize();
            self.obj().attach_buffers();

            if let Some(e) = self.obj().error() {
                eprintln!("error in WgpuArea realize, Err: {e:?}");
                return;
            }
        }

        fn unrealize(&self) {
            self.renderer.replace(None);
            self.parent_unrealize();
        }
    }

    impl GLAreaImpl for WgpuArea {
        fn resize(&self, _width: i32, _height: i32) {}

        fn render(&self, _context: &gdk::GLContext) -> glib::Propagation {
            let (width, height) = self.get_dimensions();
            if let Some(renderer) = self.renderer.borrow_mut().as_mut() {
                renderer.render(width, height);
            }
            glib::Propagation::Stop
        }
    }

    impl WgpuArea {
        fn get_dimensions(&self) -> (u32, u32) {
            let scale = self.obj().scale_factor();
            let width = self.obj().width();
            let height = self.obj().height();
            ((width * scale) as u32, (height * scale) as u32)
        }

        fn font_name(&self) -> String {
            self.obj()
                .pango_context()
                .font_description()
                .unwrap()
                .family()
                .unwrap()
                .to_string()
        }
    }
}

glib::wrapper! {
    pub struct WgpuArea(ObjectSubclass<imp::WgpuArea>)
        @extends gtk::Widget, gtk::GLArea,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for WgpuArea {
    fn default() -> Self {
        glib::Object::new()
    }
}
