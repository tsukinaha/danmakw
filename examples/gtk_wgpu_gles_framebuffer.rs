use std::cell::RefCell;

use adw::prelude::*;
use gtk::glib;
mod utils;

#[derive(Clone)]
struct Timer {
    pub milis: RefCell<f64>,
}

impl danmakw::Timer for Timer {
    fn time_milis(&self) -> f64 {
        *self.milis.borrow_mut() += 1000.0 / 165.0;
        *self.milis.borrow()
    }
}

impl Timer {
    pub fn new() -> Self {
        Self {
            milis: RefCell::new(0.0),
        }
    }
}

pub fn build_ui(application: &gtk::Application) {
    let window = adw::ApplicationWindow::builder()
        .application(application)
        .title("GtkWgpuArea 2x2 Grid")
        .default_width(800)
        .default_height(600)
        .build();

    let toolbar_view = adw::ToolbarView::new();

    let title_bar = adw::HeaderBar::builder()
        .title_widget(&gtk::Label::new(Some("WGPU Danmakw Renderer Example")))
        .build();

    toolbar_view.add_top_bar(&title_bar);

    let area = danmakw::DanmakwArea::default();
    gtk::glib::spawn_future_local(glib::clone!(
        #[weak(rename_to = area)]
        area,
        async move {
            glib::timeout_future_seconds(1).await;
            let danmakus = utils::parse_bilibili_xml(include_str!("test.xml")).unwrap();
            area.set_danmaku(danmakus);
            area.set_enable_danmaku(true);
            area.start_rendering(Timer::new());
        }
    ));

    toolbar_view.set_content(Some(&area));
    toolbar_view.add_css_class("ad");

    window.set_content(Some(&toolbar_view));
    window.present();
}

pub fn main() {
    let application = gtk::Application::new(
        Some("com.example.gtk_wgpu_gles_framebuffer"),
        Default::default(),
    );

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run();
}
