use flume::{unbounded, Receiver, Sender};
use once_cell::sync::Lazy;

use danmakw::ExportTexture;

pub struct RequestFrameChannel {
    pub tx: Sender<()>,
    pub rx: Receiver<()>,
}

pub static REQUEST_FRAME_CHANNEL: Lazy<RequestFrameChannel> = Lazy::new(|| {
    let (tx, rx) = unbounded::<()>();

    RequestFrameChannel { tx, rx }
});

pub struct ReceiveFrameChannel {
    pub tx: Sender<ExportTexture>,
    pub rx: Receiver<ExportTexture>,
}

pub static RECEIVE_FRAME_CHANNEL: Lazy<ReceiveFrameChannel> = Lazy::new(|| {
    let (tx, rx) = unbounded::<ExportTexture>();

    ReceiveFrameChannel { tx, rx }
});
