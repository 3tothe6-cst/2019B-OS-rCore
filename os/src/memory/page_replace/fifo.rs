use {
    super::*,
    alloc::{collections::VecDeque, sync::Arc},
    spin::Mutex,
};

#[derive(Default)]
pub struct FifoPageReplace {
    frames: VecDeque<(usize, Arc<Mutex<PageTableImpl>>)>,
}

impl PageReplace for FifoPageReplace {
    fn push_frame(&mut self, vaddr: usize, pt: Arc<Mutex<PageTableImpl>>) {
        println!("push vaddr: {:#x?}", vaddr);
        self.frames.push_back((vaddr, pt));
    }

    fn choose_victim(&mut self) -> Option<(usize, Arc<Mutex<PageTableImpl>>)> {
        if self.frames.is_empty() {
            return None;
        }
        loop {
            let mut frame = self.frames[0].1.lock();
            let entry = frame.get_entry(self.frames[0].0).unwrap();
            if entry.accessed() {
                entry.clear_accessed();
                drop(frame);
                let first = self.frames.pop_front().unwrap();
                self.frames.push_back(first);
            } else {
                drop(frame);
                break self.frames.pop_front();
            }
        }
    }

    fn tick(&self) {}
}
