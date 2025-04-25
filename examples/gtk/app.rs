use adw::{
    prelude::*,
    subclass::prelude::*,
};
use gtk::glib;
use gtk::prelude::*;

pub fn new_app() -> adw::Application {
    let app = adw::Application::new(
        Some("com.github.danmakuw"),
        Default::default(),
    );

    app.connect_activate(|app| {
        crate::gtk::TestWindow::new(app).present();
    });

    app
}