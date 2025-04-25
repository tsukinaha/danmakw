mod gtk_example;
mod utils;

use gtk::prelude::*;
pub use utils::parse_bilibili_xml;

fn main() {
    gtk_example::new_app().run();
}