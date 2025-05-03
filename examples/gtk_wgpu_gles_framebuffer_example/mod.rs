mod area;
mod renderer;

pub use renderer::Renderer as DanmakwAreaRenderer;

use area::DanmakwArea;
use gtk::prelude::*;
use adw::prelude::*;

pub fn build_ui(application: &gtk::Application) {
    let window = adw::ApplicationWindow::builder()
        .application(application)
        .title("GtkWgpuArea 3x3 Grid")
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

    for row in 0..3 {
        for col in 0..3 {
            let area = DanmakwArea::default();
            grid.attach(&area, col, row, 1, 1);
        }
    }

    grid.set_hexpand(true);
    grid.set_vexpand(true);

    toolbar_view.set_content(Some(&grid));

    window.set_content(Some(&toolbar_view));
    window.present();
}
