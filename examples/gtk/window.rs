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

    use std::{cell::RefCell, sync::{
        atomic::AtomicBool, Arc
    }};

    use glib::{
        WeakRef,
        subclass::InitializingObject,
    };
    use gtk::CompositeTemplate;
    use quick_xml::se;

    use crate::gtk::{
        channel::RECEIVE_FRAME_CHANNEL,
        dmabuf_texture::TextureBuilder, DanmakuArea,
    };

    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::TestWindow)]
    pub struct TestWindow {
        
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

            let toolbar_view = adw::ToolbarView::new();
                
            let title_bar = adw::HeaderBar::builder()
                .title_widget(&gtk::Label::new(Some("WGPU Danmakw Renderer Example")))
                .build();

            let danmaku_area = DanmakuArea::new();

            toolbar_view.set_content(Some(&danmaku_area));
            toolbar_view.add_top_bar(&title_bar);

            self.obj().set_content(Some(&toolbar_view));

            self.obj().start_rendering();
        }
    }

    impl WidgetImpl for TestWindow {
        
    }

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