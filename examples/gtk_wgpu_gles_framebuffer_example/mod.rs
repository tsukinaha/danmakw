// Modules
mod area;
mod renderer;

pub use renderer::Renderer as DanmakwAreaRenderer;

// Re-Exports
use area::DanmakwArea;

// Imports
use gtk::prelude::*;

pub fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("GtkWgpuArea")
        .default_width(800)
        .default_height(600)
        .build();

    let wgpu_area = DanmakwArea::default();
    window.set_child(Some(&wgpu_area));
    window.present();
}
