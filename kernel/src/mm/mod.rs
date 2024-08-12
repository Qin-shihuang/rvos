use core::ptr::addr_of;

use addr::{kva2pa, PhysAddr, VirtAddr};

use crate::config::{MEMORY_SIZE, PHYSICAL_MEMORY_START};

pub mod addr;
pub mod consts;
mod frame;
mod heap;
pub mod layout;
pub mod paging;

extern "C" {
    fn __kernel_end();
}

pub fn init() {
    heap::init();
    heap::heap_test();
    frame::init(
        kva2pa(VirtAddr(__kernel_end as usize)),
        PhysAddr(PHYSICAL_MEMORY_START + MEMORY_SIZE),
    );
    layout::print_memory_layout();
    // frame::debug_print();
}