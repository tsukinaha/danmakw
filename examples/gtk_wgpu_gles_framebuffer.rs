use adw::prelude::*;
use gtk::{
    glib,
    prelude::*,
};
mod utils;

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

    let grid = gtk::Grid::new();
    grid.set_row_homogeneous(true);
    grid.set_column_homogeneous(true);

    for row in 0..2 {
        for col in 0..2 {
            let area = danmakw::DanmakwArea::default();
            gtk::glib::spawn_future_local(glib::clone!(
                #[weak(rename_to = area)]
                area,
                async move {
                    glib::timeout_future_seconds(1).await;
                    let danmakus = utils::parse_bilibili_xml(include_str!("test.xml")).unwrap();
                    area.set_danmaku(danmakus);
                }
            ));
            grid.attach(&area, col, row, 1, 1);
        }
    }

    grid.set_hexpand(true);
    grid.set_vexpand(true);

    toolbar_view.set_content(Some(&grid));

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
