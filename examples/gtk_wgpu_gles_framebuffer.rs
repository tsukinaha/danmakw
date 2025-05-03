mod gtk_wgpu_gles_framebuffer_example;
mod utils;

use gtk::{
    glib,
    prelude::*,
};
pub use utils::parse_bilibili_xml;

fn main() -> glib::ExitCode {
    let application =
        gtk::Application::new(Some("com.github.flxzt.gtkwgpuarea"), Default::default());
    application.connect_activate(gtk_wgpu_gles_framebuffer_example::build_ui);
    application.run()
}
