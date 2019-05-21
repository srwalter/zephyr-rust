#![feature(alloc_error_handler)]

extern crate alloc;
extern crate zephyr_alloc;

use alloc::alloc::Layout;

use zephyr_alloc::ZephyrAlloc;

#[global_allocator]
static A: ZephyrAlloc = ZephyrAlloc;

use core::fmt::Write;
use zephyr_sys::syscalls;

#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    panic!("alloc of {} bytes failed", layout.size());
}

#[no_mangle]
pub extern "C" fn hello_rust() {
    writeln!(&mut std::io::Stdout, "Hello Rust writeln").unwrap();
    {
        const MSG: &str = "Hello from Rust kernel with direct kernel call\n";
        unsafe { syscalls::kernel::k_str_out(MSG.as_ptr() as *mut _, MSG.len()) };
    }
    {
        const MSG: &str = "Hello from Rust kernel with runtime-detect syscall\n";
        unsafe { syscalls::k_str_out(MSG.as_ptr() as *mut _, MSG.len()) };
    }

    writeln!(&mut std::io::Stdout, "Put some stuff on the heap").unwrap();
    let x = alloc::boxed::Box::new(1);
    writeln!(&mut std::io::Stdout, "Heap stuff: {}", *x).unwrap();
}

#[no_mangle]
pub extern "C" fn hello_rust_user() {
    {
        const MSG: &str = "Hello from Rust userspace with forced user-mode syscall\n";
        unsafe { syscalls::user::k_str_out(MSG.as_ptr() as *mut _, MSG.len()) };
    }
    writeln!(&mut std::io::Stdout, "Put some stuff on the heap").unwrap();
    let x = alloc::boxed::Box::new(1);
    writeln!(&mut std::io::Stdout, "Heap stuff: {}", *x).unwrap();

    {
        const MSG: &str = "Hello from Rust userspace with runtime-detect syscall\nNext call will crash if userspace is working.\n";
        unsafe { syscalls::k_str_out(MSG.as_ptr() as *mut _, MSG.len()) };
    }

    // This will compile, but crash if CONFIG_USERSPACE is working
    {
        const MSG: &str = "Hello from Rust userspace with direct kernel call\n";
        unsafe { syscalls::kernel::k_str_out(MSG.as_ptr() as *mut _, MSG.len()) };
    }
}
