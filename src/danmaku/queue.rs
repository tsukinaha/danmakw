use super::{
    Danmaku,
    sort::SortByTime,
};

pub struct DanmakuQueue {
    now_queue: Vec<Danmaku>,
    all_queue: Vec<Danmaku>,
}

impl Default for DanmakuQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl DanmakuQueue {
    pub fn new() -> Self {
        Self {
            now_queue: Vec::new(),
            all_queue: Vec::new(),
        }
    }

    pub fn init(&mut self, danmaku: Vec<Danmaku>, time: f64) {
        self.all_queue = danmaku;
        self.all_queue.sort_by_time();
        self.now_queue = self.all_queue.clone();
        self.pop_to_time(time);
    }

    // When the time is changed, this should be called to update the queue
    pub fn pop_to_time(&mut self, time: f64) -> Vec<Danmaku> {
        let split_index = self
            .now_queue
            .partition_point(|danmaku| danmaku.start <= time);

        self.now_queue.drain(..split_index).collect()
    }
}
