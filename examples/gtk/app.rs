use adw::{
    prelude::*,
    subclass::prelude::*,
};
use gtk::glib;
use gtk::prelude::*;

pub fn new_app() -> gtk::Application {
    let app = gtk::Application::new(
        Some("com.github.danmakuw"),
        Default::default(),
    );

    app.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title(Some("Danmakw Renderer Demo - GTK4"));
        window.set_default_size(800, 600);
        window.present();
    });

    app
}