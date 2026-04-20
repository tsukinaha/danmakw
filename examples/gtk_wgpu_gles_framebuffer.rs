use adw::prelude::*;
use gtk::glib;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    time::Duration,
};
mod utils;

pub fn build_ui(application: &gtk::Application) {
    let window = adw::ApplicationWindow::builder()
        .application(application)
        .title("WGPU Danmakw Renderer Example")
        .default_width(800)
        .default_height(600)
        .build();

    let toolbar_view = adw::ToolbarView::new();

    let title_bar = adw::HeaderBar::builder()
        .title_widget(&gtk::Label::new(Some("WGPU Danmakw Renderer Example")))
        .build();

    toolbar_view.add_top_bar(&title_bar);

    let area = danmakw::DanmakwArea::default();

    let overlay = gtk::Overlay::new();
    overlay.set_child(Some(&area));

    let adj = gtk::Adjustment::builder()
        .lower(0.0)
        .upper(100.0)
        .step_increment(1.0)
        .build();

    let label = gtk::Label::builder()
        .label("Time")
        .valign(gtk::Align::Start)
        .margin_end(8)
        .build();

    let scale = gtk::Scale::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .adjustment(&adj)
        .build();

    let hbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .valign(gtk::Align::End)
        .margin_bottom(8)
        .margin_end(8)
        .build();

    hbox.append(&label);
    hbox.append(&scale);

    let playback_clock = Rc::new(RefCell::new(danmakw::DanmakuClock::new(area.speed_factor())));
    playback_clock.borrow_mut().pause();

    let syncing_scale = Rc::new(Cell::new(false));

    scale.connect_value_changed(glib::clone!(
        #[weak(rename_to = area)]
        area,
        #[strong]
        playback_clock,
        #[strong]
        syncing_scale,
        move |scale| {
            if syncing_scale.get() {
                return;
            }

            let value = scale.value();
            let mut clock = playback_clock.borrow_mut();
            clock.set_speed_factor(area.speed_factor());
            clock.seek(value);
            drop(clock);

            area.seek(value);
        }
    ));

    glib::timeout_add_local(
        Duration::from_millis(16),
        glib::clone!(
            #[weak]
            scale,
            #[weak]
            adj,
            #[weak(rename_to = area)]
            area,
            #[strong]
            playback_clock,
            #[strong]
            syncing_scale,
            #[upgrade_or]
            glib::ControlFlow::Break,
            move || {
                let current_time = {
                    let mut clock = playback_clock.borrow_mut();
                    clock.set_speed_factor(area.speed_factor());
                    clock.time_milis().clamp(adj.lower(), adj.upper())
                };

                syncing_scale.set(true);
                scale.set_value(current_time);
                syncing_scale.set(false);

                glib::ControlFlow::Continue
            }
        ),
    );

    gtk::glib::spawn_future_local(glib::clone!(
        #[weak(rename_to = area)]
        area,
        #[weak]
        adj,
        #[strong]
        playback_clock,
        async move {
            glib::timeout_future_seconds(1).await;
            let danmakus = utils::parse_bilibili_xml(include_str!("test.xml")).unwrap();
            let max_time = danmakus.iter().map(|danmaku| danmaku.start).fold(0.0, f64::max);

            adj.set_upper(max_time.max(100.0));
            area.set_danmaku(danmakus);
            playback_clock.borrow_mut().set_speed_factor(area.speed_factor());
            playback_clock.borrow_mut().seek(0.0);
            playback_clock.borrow_mut().resume();
            area.play();
        }
    ));

    overlay.add_overlay(&hbox);

    toolbar_view.set_content(Some(&overlay));
    toolbar_view.add_css_class("ad");

    window.set_content(Some(&toolbar_view));
    window.present();
}

pub fn main() {
    let application = gtk::Application::new(
        Some("com.example.gtk_wgpu_gles_framebuffer"),
        Default::default(),
    );

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run();
}
