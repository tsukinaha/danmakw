use gtk::{
    gdk,
    glib,
    prelude::*,
    subclass::prelude::*,
};
use std::cell::RefCell;

mod imp {
    use crate::gtk_wgpu_gles_framebuffer_example::DanmakwAreaRenderer;

    use super::*;

    #[derive(Default)]
    pub struct DanmakwArea {
        renderer: RefCell<Option<DanmakwAreaRenderer>>,
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

    impl WidgetImpl for DanmakwArea {
        fn realize(&self) {
            self.parent_realize();
            self.obj().attach_buffers();

            if let Some(e) = self.obj().error() {
                eprintln!("error in DanmakwArea realize, Err: {e:?}");
            }
        }

        fn unrealize(&self) {
            self.renderer.replace(None);
            self.parent_unrealize();
        }
    }

    impl GLAreaImpl for DanmakwArea {
        fn resize(&self, _width: i32, _height: i32) {}

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
