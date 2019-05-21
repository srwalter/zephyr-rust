#![feature(allocator_api)]
#![no_std]

extern crate zephyr_sys;
extern crate alloc;

use alloc::alloc::{GlobalAlloc, Layout};

use zephyr_sys::raw;
use zephyr_sys::ctypes;

pub struct ZephyrAlloc;

unsafe impl GlobalAlloc for ZephyrAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        raw::k_malloc(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: * mut u8, _layout: Layout) {
        raw::k_free(ptr as *mut ctypes::c_void);
    }
}
