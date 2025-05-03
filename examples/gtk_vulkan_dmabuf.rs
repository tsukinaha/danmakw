mod gtk_vulkan_dmabuf_example;
mod utils;

use gtk::prelude::*;
pub use utils::parse_bilibili_xml;

fn main() {
    gtk_vulkan_dmabuf_example::new_app().run();
}
