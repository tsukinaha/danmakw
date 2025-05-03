use glib::Object;
use gtk::{
    glib,
    prelude::*,
};

use adw::{
    prelude::*,
    subclass::prelude::*,
};

use super::{
    Properties,
    RendererEvent,
    channel::REQUEST_FRAME_CHANNEL,
    renderer::Renderer,
};

pub mod imp {
    use std::cell::RefCell;

    use gtk::{
        TickCallbackId,
        gdk,
        glib,
        prelude::*,
    };

    use crate::gtk_vulkan_dmabuf_example::{
        Properties,
        RendererEvent,
        channel::{
            RECEIVE_FRAME_CHANNEL,
            REQUEST_FRAME_CHANNEL,
        },
    };

    use super::*;

    const RGBA32: u32 = 875708993;

    // Object holding the state
    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::DanmakuArea)]
    pub struct DanmakuArea {
        #[property(get, set = Self::set_font_size)]
        pub font_size: RefCell<u32>,
        #[property(get, set = Self::set_speed_factor)]
        pub speed_factor: RefCell<f64>,
        #[property(get, set = Self::set_row_spacing)]
        pub row_spacing: RefCell<u32>,
        #[property(get, set = Self::set_max_rows)]
        pub max_rows: RefCell<u32>,
        #[property(get, set = Self::set_top_padding)]
        pub top_padding: RefCell<u32>,
        #[property(get, set = Self::set_paused)]
        pub paused: RefCell<bool>,
        #[property(get, set = Self::set_time_milis)]
        pub time_milis: RefCell<f64>,
        #[property(get, set = Self::set_font_name)]
        pub font_name: RefCell<String>,

        texture: RefCell<Option<gdk::Texture>>,

        render_loop_callback_id: RefCell<Option<TickCallbackId>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DanmakuArea {
        const NAME: &'static str = "DanmakuArea";
        type Type = super::DanmakuArea;
        type ParentType = adw::Bin;
    }

    #[glib::derived_properties]
    impl ObjectImpl for DanmakuArea {
        fn constructed(&self) {
            self.parent_constructed();
            self.set_default_font();

            glib::spawn_future_local(glib::clone!(
                #[weak(rename_to = imp)]
                self,
                async move {
                    while let Ok(tex_buf) = RECEIVE_FRAME_CHANNEL.rx.recv_async().await {
                        unsafe {
                            let builder = gdk::DmabufTextureBuilder::new();
                            builder.set_display(&gdk::Display::default().unwrap());
                            builder.set_fd(0, tex_buf.fd);
                            builder.set_fourcc(RGBA32);
                            builder.set_modifier(0);
                            builder.set_width(tex_buf.size.width);
                            builder.set_height(tex_buf.size.height);
                            builder.set_n_planes(1);
                            builder.set_offset(0, 0);
                            builder.set_stride(0, tex_buf.row_stride);
                            builder.set_premultiplied(false);
                            imp.texture.replace(Some(builder.build().unwrap()));
                            imp.obj().queue_draw();
                        }
                    }
                }
            ));

            self.obj().start_render_loop();

            REQUEST_FRAME_CHANNEL
                .tx
                .send(RendererEvent::ChangeProperties(Properties::StartRendering(
                    (),
                )))
                .unwrap();
        }
    }

    impl WidgetImpl for DanmakuArea {
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            self.parent_snapshot(snapshot);

            if let Some(texture) = self.texture.borrow().as_ref() {
                texture.snapshot(
                    snapshot,
                    self.obj().width() as f64,
                    self.obj().height() as f64,
                );
            }
        }
    }

    impl BinImpl for DanmakuArea {}

    impl DanmakuArea {
        pub fn set_default_font(&self) {
            let font = self
                .obj()
                .pango_context()
                .font_description()
                .unwrap()
                .family()
                .unwrap();
            self.obj().set_font_name(font);
        }

        pub fn set_font_name(&self, font_name: String) {
            self.font_name.replace(font_name.to_owned());
            REQUEST_FRAME_CHANNEL
                .tx
                .send(RendererEvent::ChangeProperties(Properties::SetFontName(
                    font_name,
                )))
                .unwrap();
        }

        pub fn set_font_size(&self, font_size: u32) {
            self.font_size.replace(font_size);
            REQUEST_FRAME_CHANNEL
                .tx
                .send(RendererEvent::ChangeProperties(Properties::SetFontSize(
                    font_size,
                )))
                .unwrap();
        }

        pub fn set_speed_factor(&self, speed_factor: f64) {
            self.speed_factor.replace(speed_factor);
            REQUEST_FRAME_CHANNEL
                .tx
                .send(RendererEvent::ChangeProperties(Properties::SetSpeedFactor(
                    speed_factor,
                )))
                .unwrap();
        }

        pub fn set_row_spacing(&self, row_spacing: u32) {
            self.row_spacing.replace(row_spacing);
            REQUEST_FRAME_CHANNEL
                .tx
                .send(RendererEvent::ChangeProperties(Properties::SetRowSpacing(
                    row_spacing,
                )))
                .unwrap();
        }

        pub fn set_top_padding(&self, top_padding: u32) {
            self.top_padding.replace(top_padding);
            REQUEST_FRAME_CHANNEL
                .tx
                .send(RendererEvent::ChangeProperties(Properties::SetTopPadding(
                    top_padding,
                )))
                .unwrap();
        }

        pub fn set_max_rows(&self, max_rows: u32) {
            self.max_rows.replace(max_rows);
            REQUEST_FRAME_CHANNEL
                .tx
                .send(RendererEvent::ChangeProperties(Properties::SetMaxRows(
                    max_rows as usize,
                )))
                .unwrap();
        }

        pub fn set_paused(&self, paused: bool) {
            self.paused.replace(paused);
            if paused {
                REQUEST_FRAME_CHANNEL
                    .tx
                    .send(RendererEvent::ChangeProperties(Properties::PauseRendering(
                        (),
                    )))
                    .unwrap();
            } else {
                REQUEST_FRAME_CHANNEL
                    .tx
                    .send(RendererEvent::ChangeProperties(Properties::StartRendering(
                        (),
                    )))
                    .unwrap();
            }
        }

        // This method will discard the danmakus between the last value and the current value
        // You should not bind this property to video time
        pub fn set_time_milis(&self, time_milis: f64) {
            self.time_milis.replace(time_milis);
            REQUEST_FRAME_CHANNEL
                .tx
                .send(RendererEvent::ChangeProperties(Properties::SetTimeMilis(
                    time_milis,
                )))
                .unwrap();
        }

        pub fn set_render_loop_callback_id(&self, callback_id: TickCallbackId) {
            self.render_loop_callback_id.replace(Some(callback_id));
        }

        pub fn get_render_loop_callback_id(&self) -> Option<TickCallbackId> {
            self.render_loop_callback_id.take()
        }
    }
}

glib::wrapper! {
    pub struct DanmakuArea(ObjectSubclass<imp::DanmakuArea>)
        @extends adw::Bin, gtk::Window, gtk::Widget;
}

impl Default for DanmakuArea {
    fn default() -> Self {
        Self::new()
    }
}

impl DanmakuArea {
    pub fn new() -> Self {
        Object::new()
    }

    fn start_render_loop(&self) {
        glib::spawn_future_local(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            async move {
                let mut renderer = Renderer::new().await;

                let danmakus =
                    crate::utils::parse_bilibili_xml(include_str!("../test.xml")).unwrap();
                renderer.init(danmakus);

                renderer.set_font_name(obj.font_name());

                while let Ok(event) = REQUEST_FRAME_CHANNEL.rx.recv_async().await {
                    match event {
                        RendererEvent::RequestFrame(width, height) => {
                            if width == 0 || height == 0 {
                                continue;
                            }
                            let instant = std::time::Instant::now();
                            renderer.render(width, height).await;
                            dbg!(instant.elapsed());
                        }
                        RendererEvent::ChangeProperties(p) => match p {
                            Properties::SetFontSize(size) => {
                                renderer.set_font_size(size);
                            }
                            Properties::SetSpeedFactor(speed_factor) => {
                                renderer.set_speed_factor(speed_factor);
                            }
                            Properties::SetRowSpacing(row_spacing) => {
                                renderer.set_row_spacing(row_spacing);
                            }
                            Properties::SetTopPadding(top_padding) => {
                                renderer.set_top_padding(top_padding);
                            }
                            Properties::SetMaxRows(max_rows) => {
                                renderer.set_max_rows(max_rows);
                            }
                            Properties::PauseRendering(()) => {
                                obj.stop_rendering();
                            }
                            Properties::StartRendering(()) => {
                                obj.start_rendering();
                            }
                            Properties::SetDanmaku(danmaku) => {
                                renderer.init(danmaku);
                            }
                            Properties::SetTimeMilis(time) => {
                                renderer.set_video_time(time);
                            }
                            Properties::SetFontName(font_name) => {
                                renderer.set_font_name(font_name);
                            }
                            Properties::SetVideoSpeed(speed) => {
                                renderer.set_video_speed(speed);
                            }
                        },
                    }
                }
            }
        ));
    }

    fn start_rendering(&self) {
        let callback_id = self.add_tick_callback(move |area, _a| {
            let width = area.width();
            let height = area.height();
            REQUEST_FRAME_CHANNEL
                .tx
                .send(
                    crate::gtk_vulkan_dmabuf_example::RendererEvent::RequestFrame(
                        width as u32,
                        height as u32,
                    ),
                )
                .unwrap();
            glib::ControlFlow::Continue
        });

        self.imp().set_render_loop_callback_id(callback_id);
    }

    fn stop_rendering(&self) {
        if let Some(callback_id) = self.imp().get_render_loop_callback_id() {
            callback_id.remove();
        }
    }
}
