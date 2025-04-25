mod gtk;
mod utils;

use gdk::gio::prelude::ApplicationExtManual;
pub use utils::parse_bilibili_xml;

fn main() {
    gtk::new_app().run();
}