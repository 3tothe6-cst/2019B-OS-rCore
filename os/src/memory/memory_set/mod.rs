use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::ops::DerefMut;

use spin::Mutex;

use area::MemoryArea;
use attr::MemoryAttr;
use handler::{Linear, MemoryHandler};

use crate::consts::*;
use crate::memory::access_pa_via_va;
use crate::memory::paging::{PageRange, PageTableImpl};

pub mod area;
pub mod attr;
pub mod handler;

pub struct MemorySet {
    areas: Vec<MemoryArea>,
    page_table: Arc<Mutex<PageTableImpl>>,
}

impl MemorySet {
    pub fn clone(&mut self) -> Self {
        // 创建一个新的页目录
        let mut new_page_table = PageTableImpl::new_bare();
        let Self {
            ref mut page_table,
            ref areas,
            ..
        } = self;
        // 遍历自己的所有页面
        for area in areas.iter() {
            for page in PageRange::new(area.start, area.end) {
                // 创建一个新的页
                // 将原页的内容复制到新页，同时进行映射
                area.handler.clone_map(
                    &mut new_page_table,
                    page_table.lock().deref_mut(),
                    page,
                    &area.attr,
                );
            }
        }
        MemorySet {
            areas: areas.clone(),
            page_table: Arc::new(Mutex::new(new_page_table)),
        }
    }
    pub fn push(
        &mut self,
        start: usize,
        end: usize,
        attr: MemoryAttr,
        handler: impl MemoryHandler,
        data: Option<(usize, usize)>,
    ) {
        assert!(start <= end, "invalid memory area!");
        assert!(self.test_free_area(start, end), "memory area overlap!");
        let area = MemoryArea::new(start, end, Box::new(handler), attr);
        area.map(self.page_table.clone());
        if let Some((src, length)) = data {
            area.page_copy(self.page_table.clone(), src, length);
        }
        self.areas.push(area);
    }
    fn test_free_area(&self, start: usize, end: usize) -> bool {
        self.areas
            .iter()
            .find(|area| area.is_overlap_with(start, end))
            .is_none()
    }
    pub unsafe fn activate(&self) {
        self.page_table.lock().activate();
    }
    pub fn new() -> Self {
        let mut memory_set = MemorySet {
            areas: Vec::new(),
            page_table: Arc::new(Mutex::new(PageTableImpl::new_bare())),
        };
        memory_set.map_kernel_and_physical_memory();
        memory_set
    }
    pub fn map_kernel_and_physical_memory(&mut self) {
        extern "C" {
            fn stext();
            fn etext();
            fn srodata();
            fn erodata();
            fn sdata();
            fn edata();
            fn sbss();
            fn ebss();
            fn end();
        }
        let offset = PHYSICAL_MEMORY_OFFSET;
        // 各段全部采用偏移量固定的线性映射
        // .text R|X
        self.push(
            stext as usize,
            etext as usize,
            MemoryAttr::new().set_readonly().set_execute(),
            Linear::new(offset),
            None,
        );
        // .rodata R
        self.push(
            srodata as usize,
            erodata as usize,
            MemoryAttr::new().set_readonly(),
            Linear::new(offset),
            None,
        );
        // .data R|W
        self.push(
            sdata as usize,
            edata as usize,
            MemoryAttr::new(),
            Linear::new(offset),
            None,
        );
        // .bss R|W
        self.push(
            sbss as usize,
            ebss as usize,
            MemoryAttr::new(),
            Linear::new(offset),
            None,
        );
        // 物理内存 R|W
        self.push(
            (end as usize / PAGE_SIZE + 1) * PAGE_SIZE,
            access_pa_via_va(PHYSICAL_MEMORY_END),
            MemoryAttr::new(),
            Linear::new(offset),
            None,
        );
    }
    pub fn token(&self) -> usize {
        self.page_table.lock().token()
    }
}
