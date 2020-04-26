use super::Tid;
use alloc::vec::Vec;
use core::cmp::min;

pub trait Scheduler {
    fn push(&mut self, tid: Tid);
    fn pop(&mut self) -> Option<Tid>;
    fn tick(&mut self) -> bool;
    fn exit(&mut self, tid: Tid);
}

#[derive(Default)]
struct RRInfo {
    valid: bool,
    time: usize,
    prev: usize,
    next: usize,
}

pub struct RRScheduler {
    threads: Vec<RRInfo>,
    max_time: usize,
    current: usize,
}

impl RRScheduler {
    pub fn new(max_time_slice: usize) -> Self {
        let mut rr = RRScheduler {
            threads: Vec::default(),
            max_time: max_time_slice,
            current: 0,
        };
        rr.threads.push(RRInfo {
            valid: false,
            time: 0,
            prev: 0,
            next: 0,
        });
        rr
    }
}
impl Scheduler for RRScheduler {
    fn push(&mut self, tid: Tid) {
        let tid = tid + 1;
        if tid + 1 > self.threads.len() {
            self.threads.resize_with(tid + 1, Default::default);
        }

        if self.threads[tid].time == 0 {
            self.threads[tid].time = self.max_time;
        }

        let prev = self.threads[0].prev;
        self.threads[tid].valid = true;
        self.threads[prev].next = tid;
        self.threads[tid].prev = prev;
        self.threads[0].prev = tid;
        self.threads[tid].next = 0;
    }

    fn pop(&mut self) -> Option<Tid> {
        let ret = self.threads[0].next;
        if ret != 0 {
            let next = self.threads[ret].next;
            let prev = self.threads[ret].prev;
            self.threads[next].prev = prev;
            self.threads[prev].next = next;
            self.threads[ret].prev = 0;
            self.threads[ret].next = 0;
            self.threads[ret].valid = false;
            self.current = ret;
            Some(ret - 1)
        } else {
            None
        }
    }

    // 当前线程的可用时间片 -= 1
    fn tick(&mut self) -> bool {
        let tid = self.current;
        if tid != 0 {
            self.threads[tid].time -= 1;
            if self.threads[tid].time == 0 {
                return true;
            } else {
                return false;
            }
        }
        return true;
    }

    fn exit(&mut self, tid: Tid) {
        let tid = tid + 1;
        if self.current == tid {
            self.current = 0;
        }
    }
}

pub struct StridePassInfo {
    stride: usize,
    pub pass: usize,
}

pub struct StrideScheduler {
    threads: Vec<(Tid, StridePassInfo)>,
    current: Option<Tid>,
}

impl StrideScheduler {
    pub fn new() -> Self {
        Self {
            threads: Vec::new(),
            current: None,
        }
    }

    pub fn find_mut(&mut self, tid: Tid) -> Option<&mut StridePassInfo> {
        for t in &mut self.threads {
            if t.0 == tid {
                return Some(&mut t.1)
            }
        }
        None
    }
}

impl Scheduler for StrideScheduler {
    fn push(&mut self, tid: Tid) {
        self.threads.push((tid, StridePassInfo { stride: 0, pass: 65536 }));
    }

    fn pop(&mut self) -> Option<Tid> {
        let mut result: Option<&mut (usize, StridePassInfo)> = None;
        for t in &mut self.threads {
            if result.is_none() || t.1.stride < result.as_mut().unwrap().1.stride {
                result = Some(t);
            }
        }
        result.map(|x| x.0)
    }

    fn tick(&mut self) -> bool {
        if let Some(current) = self.current {
            let sp = self.find_mut(current).unwrap();
            sp.stride += sp.pass;
        }
        true
    }

    fn exit(&mut self, tid: Tid) {
        let idx = self.threads.iter_mut().enumerate().find(|x| (x.1).0 == tid).map(|(idx, _)| idx);
        if let Some(idx) = idx {
            self.threads.remove(idx);
        }
    }
}
