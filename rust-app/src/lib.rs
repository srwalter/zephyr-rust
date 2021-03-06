use core::fmt::Write;

#[no_mangle]
pub extern "C" fn hello_rust() {
    writeln!(&mut std::io::Stdout, "Hello Rust writeln").unwrap();
    zephyr::kernel::k_str_out("Hello from Rust kernel with direct kernel call\n");
    zephyr::any::k_str_out("Hello from Rust kernel with runtime-detect syscall\n");

    {
        let boxed = Box::new(1u8);
        writeln!(&mut std::io::Stdout, "Boxed value {}", boxed).unwrap();
    }

    {
        let io_results: [std::io::Result<()>; 2] = [Ok(()), Err(std::io::Error)];

        for io in &io_results {
            if io.is_ok() {
                writeln!(&mut std::io::Stdout, "io::Result is Ok").unwrap();
            } else {
                writeln!(&mut std::io::Stdout, "io::Result is Error").unwrap();
            }
        }
    }

    // test std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive}
    {
        let a: [u8; 4] = [1, 2, 3, 4];
        let len = a.iter().len();
        for _ in &a[0..len] {}
        for _ in &a[0..=(len - 1)] {}
        for _ in &a[..] {}
        for _ in &a[0..] {}
        for _ in &a[..len] {}
        for _ in &a[..=(len - 1)] {}
    }

    zephyr::kernel::k_thread_user_mode_enter(|| {
        zephyr::user::k_str_out("Hello from Rust userspace with forced user-mode syscall\n");
        zephyr::any::k_str_out("Hello from Rust userspace with runtime-detect syscall\nNext call will crash if userspace is working.\n");

        // This will compile, but crash if CONFIG_USERSPACE is working
        zephyr::kernel::k_str_out("Hello from Rust userspace with direct kernel call\n");
    });
}
