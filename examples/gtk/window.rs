use adw::{
    prelude::*,
    subclass::prelude::*,
};
use gtk::{
    Adjustment,
    glib,
    template_callbacks,
};

use super::{
    channel::REQUEST_FRAME_CHANNEL,
    renderer::Renderer,
};

mod imp {

    use std::sync::{
        Arc,
        atomic::AtomicBool,
    };

    use glib::{
        WeakRef,
        subclass::InitializingObject,
    };
    use gtk::CompositeTemplate;
    use quick_xml::se;

    use crate::gtk::{
        channel::RECEIVE_FRAME_CHANNEL,
        dmabuf_texture::TextureBuilder,
    };

    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::TestWindow)]
    pub struct TestWindow {
        pub picture: gtk::Picture,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TestWindow {
        const NAME: &'static str = "YoakeWindow";
        type Type = super::TestWindow;
        type ParentType = adw::ApplicationWindow;
    }

    #[glib::derived_properties]
    impl ObjectImpl for TestWindow {
        fn constructed(&self) {
            self.parent_constructed();

            self.obj().set_content(Some(&self.picture));

            self.obj().start_rendering();

            self.obj().add_tick_callback(|window, _a| {
                let width = window.imp().picture.width();
                let height = window.imp().picture.height();
                REQUEST_FRAME_CHANNEL
                    .tx
                    .send((width as u32, height as u32))
                    .unwrap();
                glib::ControlFlow::Continue
            });

            glib::spawn_future_local(glib::clone!(
                #[weak(rename_to = imp)]
                self,
                async move {
                    while let Ok(tex_buf) = RECEIVE_FRAME_CHANNEL.rx.recv_async().await {
                        unsafe {
                            let dmabuf_texture = TextureBuilder::new()
                                .display(&gdk::Display::default().unwrap())
                                .fd(0, tex_buf.fd)
                                .fourcc(875709016)
                                .modifier(0)
                                .n_planes(1)
                                .width(tex_buf.size.width)
                                .height(tex_buf.size.height)
                                .offset(0, 0)
                                .stride(0, tex_buf.row_stride)
                                .build()
                                .unwrap();

                            imp.picture.set_paintable(Some(&dmabuf_texture));
                        }
                    }
                }
            ));
        }
    }

    impl WidgetImpl for TestWindow {}

    impl WindowImpl for TestWindow {}

    impl ApplicationWindowImpl for TestWindow {}

    impl AdwApplicationWindowImpl for TestWindow {}
}

glib::wrapper! {
    pub struct TestWindow(ObjectSubclass<imp::TestWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget, @implements gtk::Accessible;
}

#[template_callbacks]
impl TestWindow {
    pub fn new(app: &adw::Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    pub fn start_rendering(&self) {
        glib::spawn_future_local(async {
            let mut renderer = Renderer::new().await;

            let danmakus = crate::utils::parse_bilibili_xml(include_str!("../test.xml")).unwrap();
            renderer.init(danmakus);

            while let Ok((width, height)) = REQUEST_FRAME_CHANNEL.rx.recv_async().await {
                if width == 0 || height == 0 {
                    continue;
                }
                renderer.render(width , height).await;
            }
        });
    }
}