use gtk::{
    gdk,
    glib,
    prelude::*,
    subclass::prelude::*,
};
use std::cell::RefCell;

mod imp {
    use std::panic;

    use gtk::TickCallbackId;

    use crate::gtk_wgpu_gles_framebuffer_example::DanmakwAreaRenderer;

    use super::*;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::DanmakwArea)]
    pub struct DanmakwArea {
        #[property(get, set = Self::set_font_size)]
        pub font_size: RefCell<u32>,
        #[property(get, set = Self::set_speed_factor)]
        pub speed_factor: RefCell<f64>,
        #[property(get, set = Self::set_row_spacing)]
        pub row_spacing: RefCell<u32>,
        #[property(get, set = Self::set_max_rows)]
        pub max_rows: RefCell<u32>,
        #[property(get, set = Self::set_top_padding)]
        pub top_padding: RefCell<u32>,
        #[property(get, set = Self::set_paused)]
        pub paused: RefCell<bool>,
        #[property(get, set = Self::set_time_milis)]
        pub time_milis: RefCell<f64>,
        #[property(get, set = Self::set_font_name)]
        pub font_name: RefCell<String>,

        pub renderer: RefCell<Option<DanmakwAreaRenderer>>,
        render_loop_callback_id: RefCell<Option<TickCallbackId>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DanmakwArea {
        const NAME: &'static str = "DanmakwArea";
        type Type = super::DanmakwArea;
        type ParentType = gtk::GLArea;
    }

    impl ObjectImpl for DanmakwArea {
        fn constructed(&self) {
            self.parent_constructed();

            let provider = gtk::CssProvider::new();
            provider.load_from_string(
                "
                .danmakw-area {
                    transform: scaleY(-1);
                }",
            );
            gtk::style_context_add_provider_for_display(
                &gdk::Display::default().expect("Could not connect to a display."),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );

            self.obj().add_css_class("danmakw-area");

            load_epoxy();

            glib::spawn_future_local(glib::clone!(
                #[weak(rename_to = imp)]
                self,
                async move {
                    imp.obj().attach_buffers();
                    let mut renderer = DanmakwAreaRenderer::new().await;
                    renderer.danmaku_renderer.set_font_name(imp.font_name());
                    imp.renderer.replace(Some(renderer));
                }
            ));
        }
    }

    impl WidgetImpl for DanmakwArea {
        fn realize(&self) {
            self.parent_realize();
            self.obj().attach_buffers();

            if let Some(e) = self.obj().error() {
                panic!("Failed to create GLArea: {e}");
            }

            self.obj().start_rendering();
        }

        fn unrealize(&self) {
            self.obj().stop_rendering();
            self.renderer.take();
            self.parent_unrealize();
        }
    }

    impl GLAreaImpl for DanmakwArea {
        fn resize(&self, width: i32, height: i32) {
            if let Some(renderer) = self.renderer.borrow_mut().as_mut() {
                renderer.resize(width as u32, height as u32);
            }
        }

        fn render(&self, _context: &gdk::GLContext) -> glib::Propagation {
            let (width, height) = self.get_dimensions();
            if let Some(renderer) = self.renderer.borrow_mut().as_mut() {
                renderer.render(width, height);
            }
            glib::Propagation::Stop
        }
    }

    impl DanmakwArea {
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

        pub fn set_render_loop_callback_id(&self, callback_id: TickCallbackId) {
            self.render_loop_callback_id.replace(Some(callback_id));
        }

        pub fn get_render_loop_callback_id(&self) -> Option<TickCallbackId> {
            self.render_loop_callback_id.take()
        }

        fn set_font_size(&self, font_size: u32) {
            self.font_size.replace(font_size);
            if let Some(renderer) = self.renderer.borrow_mut().as_mut() {
                renderer.danmaku_renderer.set_font_size(font_size as f32);
            }
        }

        fn set_speed_factor(&self, speed_factor: f64) {
            self.speed_factor.replace(speed_factor);
            if let Some(renderer) = self.renderer.borrow_mut().as_mut() {
                renderer.danmaku_renderer.set_speed_factor(speed_factor);
            }
        }

        fn set_row_spacing(&self, row_spacing: u32) {
            self.row_spacing.replace(row_spacing);
            if let Some(renderer) = self.renderer.borrow_mut().as_mut() {
                renderer
                    .danmaku_renderer
                    .set_row_spacing(row_spacing as f32);
            }
        }

        fn set_max_rows(&self, max_rows: u32) {
            self.max_rows.replace(max_rows);
            if let Some(renderer) = self.renderer.borrow_mut().as_mut() {
                renderer.danmaku_renderer.set_max_rows(max_rows as usize);
            }
        }

        fn set_top_padding(&self, top_padding: u32) {
            self.top_padding.replace(top_padding);
            if let Some(renderer) = self.renderer.borrow_mut().as_mut() {
                renderer
                    .danmaku_renderer
                    .set_top_padding(top_padding as f32);
            }
        }

        fn set_paused(&self, paused: bool) {
            self.paused.replace(paused);
            if let Some(renderer) = self.renderer.borrow_mut().as_mut() {
                renderer.danmaku_renderer.set_paused(paused);
            }
        }

        fn set_time_milis(&self, time_milis: f64) {
            self.time_milis.replace(time_milis);
            if let Some(renderer) = self.renderer.borrow_mut().as_mut() {
                renderer.danmaku_renderer.set_video_time(time_milis);
            }
        }

        fn set_font_name(&self, font_name: String) {
            self.font_name.replace(font_name);
            if let Some(renderer) = self.renderer.borrow_mut().as_mut() {
                renderer.danmaku_renderer.set_font_name(self.font_name());
            }
        }
    }

    fn load_epoxy() {
        #[cfg(target_os = "macos")]
        {
            let is_angle_loaded = unsafe {
                [
                    libloading::os::unix::Library::new("libEGL.dylib"),
                    libloading::os::unix::Library::new("libGLESv2.dylib"),
                ]
            }
            .iter()
            .fold(true, |acc, new| acc && new.is_ok());
            if !is_angle_loaded {
                panic!("The ANGLE library must be loaded for this example to work");
            }
        }

        #[cfg(target_os = "macos")]
        let library = unsafe { libloading::os::unix::Library::new("libepoxy.0.dylib") }.unwrap();
        #[cfg(all(unix, not(target_os = "macos")))]
        let library = unsafe { libloading::os::unix::Library::new("libepoxy.so.0") }.unwrap();
        #[cfg(windows)]
        let library = libloading::os::windows::Library::open_already_loaded("libepoxy-0.dll")
            .or_else(|_| libloading::os::windows::Library::open_already_loaded("epoxy-0.dll"))
            .unwrap();

        epoxy::load_with(|name| {
            unsafe { library.get(name.as_bytes()) }
                .map(|symbol| *symbol)
                .unwrap_or_else(|e| {
                    eprintln!("failed to init epoxy, Err: {e:?}");
                    std::ptr::null()
                })
        });
    }
}

glib::wrapper! {
    pub struct DanmakwArea(ObjectSubclass<imp::DanmakwArea>)
        @extends gtk::Widget, gtk::GLArea,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for DanmakwArea {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl DanmakwArea {
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn start_rendering(&self) {
        let id = self.add_tick_callback(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[upgrade_or]
            glib::ControlFlow::Continue,
            move |_, _| {
                obj.queue_draw();
                glib::ControlFlow::Continue
            }
        ));

        self.imp().set_render_loop_callback_id(id);
    }

    pub fn stop_rendering(&self) {
        if let Some(id) = self.imp().get_render_loop_callback_id() {
            id.remove();
        }
    }

    pub fn set_danmaku(&self, danmaku: Vec<crate::Danmaku>) {
        if let Some(renderer) = self.imp().renderer.borrow_mut().as_mut() {
            renderer.danmaku_renderer.init(danmaku);
        }
    }
}
