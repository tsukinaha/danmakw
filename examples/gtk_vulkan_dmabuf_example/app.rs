use adw::prelude::*;

pub fn new_app() -> adw::Application {
    let app = adw::Application::new(Some("com.github.danmakuw"), Default::default());

    app.connect_activate(|app| {
        crate::gtk_vulkan_dmabuf_example::TestWindow::new(app).present();
    });

    app
}
