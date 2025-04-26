use glib::Object;
use gtk::{
    gio,
    glib,
};

pub mod imp {
    use std::cell::RefCell;

    use adw::subclass::prelude::*;

    use gtk::{
        gdk,
        glib,
        prelude::*,
    };

    use crate::gtk_example::channel::{
        RECEIVE_FRAME_CHANNEL,
        REQUEST_FRAME_CHANNEL,
    };

    const RGBA32: u32 = 875708993;

    // Object holding the state
    #[derive(Default)]
    pub struct DanmakuArea {
        pub texture: RefCell<Option<gdk::Texture>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DanmakuArea {
        const NAME: &'static str = "DanmakuArea";
        type Type = super::DanmakuArea;
        type ParentType = adw::Bin;
    }

    impl ObjectImpl for DanmakuArea {
        fn constructed(&self) {
            self.parent_constructed();

            self.obj().add_tick_callback(move |area, _a| {
                let width = area.width();
                let height = area.height();
                REQUEST_FRAME_CHANNEL
                    .tx
                    .send((width as u32, height as u32))
                    .unwrap();
                glib::ControlFlow::Continue
            });

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

    impl DanmakuArea {}
}

glib::wrapper! {
    pub struct DanmakuArea(ObjectSubclass<imp::DanmakuArea>)
        @extends gtk::ApplicationWindow, adw::Bin, gtk::Window, gtk::Widget ,adw::NavigationPage,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
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
}
