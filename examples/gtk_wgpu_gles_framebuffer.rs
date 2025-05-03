mod gtk_wgpu_gles_framebuffer_example;
mod utils;

use gtk::prelude::*;
pub use utils::parse_bilibili_xml;
use gtk::glib;

fn main() -> glib::ExitCode {
    let application =
        gtk::Application::new(Some("com.github.flxzt.gtkwgpuarea"), Default::default());
    application.connect_activate(gtk_wgpu_gles_framebuffer_example::build_ui);
    application.run()
}