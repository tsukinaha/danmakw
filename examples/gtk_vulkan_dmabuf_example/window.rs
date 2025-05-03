use adw::{
    prelude::*,
    subclass::prelude::*,
};
use gtk::{
    glib,
    template_callbacks,
};

mod imp {

    use crate::gtk_vulkan_dmabuf_example::DanmakuArea;

    use super::*;

    #[derive(Debug, Default)]
    pub struct TestWindow {}

    #[glib::object_subclass]
    impl ObjectSubclass for TestWindow {
        const NAME: &'static str = "YoakeWindow";
        type Type = super::TestWindow;
        type ParentType = adw::ApplicationWindow;
    }

    impl ObjectImpl for TestWindow {
        fn constructed(&self) {
            self.parent_constructed();

            let toolbar_view = adw::ToolbarView::new();

            let title_bar = adw::HeaderBar::builder()
                .title_widget(&gtk::Label::new(Some("WGPU Danmakw Renderer Example")))
                .build();

            let danmaku_area = DanmakuArea::new();

            let offload = gtk::GraphicsOffload::new(Some(&danmaku_area));
            offload.set_enabled(gtk::GraphicsOffloadEnabled::Enabled);
            offload.set_black_background(false);
            offload.set_vexpand(true);

            toolbar_view.set_content(Some(&offload));
            toolbar_view.add_top_bar(&title_bar);

            self.obj().set_content(Some(&toolbar_view));
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
}
