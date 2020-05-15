use alloc::collections::VecDeque;

use spin::Mutex;

use crate::process::{current_tid, park, wake_up, Tid};

#[derive(Default)]
pub struct Condvar {
    wait_queue: Mutex<VecDeque<Tid>>,
}

impl Condvar {
    pub fn new() -> Self {
        Condvar::default()
    }

    pub fn wait(&self) {
        self.wait_queue.lock().push_back(current_tid());
        park();
    }

    pub fn notify(&self) {
        let tid = self.wait_queue.lock().pop_front();
        if let Some(tid) = tid {
            wake_up(tid);
        }
        /* yield_now(); */
    }
}
