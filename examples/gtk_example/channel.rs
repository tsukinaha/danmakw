use flume::{
    Receiver,
    Sender,
    unbounded,
};
use once_cell::sync::Lazy;

use danmakw::ExportTextureBuf;

pub struct RequestFrameChannel {
    pub tx: Sender<(u32, u32)>,
    pub rx: Receiver<(u32, u32)>,
}

pub static REQUEST_FRAME_CHANNEL: Lazy<RequestFrameChannel> = Lazy::new(|| {
    let (tx, rx) = unbounded::<(u32, u32)>();

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
