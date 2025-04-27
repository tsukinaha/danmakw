use flume::{
    Receiver,
    Sender,
    unbounded,
};
use once_cell::sync::Lazy;

use danmakw::ExportTextureBuf;
use super::RendererEvent;

pub struct RequestFrameChannel {
    pub tx: Sender<RendererEvent>,
    pub rx: Receiver<RendererEvent>,
}

pub static REQUEST_FRAME_CHANNEL: Lazy<RequestFrameChannel> = Lazy::new(|| {
    let (tx, rx) = unbounded::<RendererEvent>();

    RequestFrameChannel { tx, rx }
});

pub struct ReceiveFrameChannel {
    pub tx: Sender<ExportTextureBuf>,
    pub rx: Receiver<ExportTextureBuf>,
}

pub static RECEIVE_FRAME_CHANNEL: Lazy<ReceiveFrameChannel> = Lazy::new(|| {
    let (tx, rx) = unbounded::<ExportTextureBuf>();

    ReceiveFrameChannel { tx, rx }
});
