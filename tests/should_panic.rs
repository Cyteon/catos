// Dont link standard library
#![no_std]
// Dont use normal entry point
#![no_main]

use cat_os::serial_println;
use core::panic::PanicInfo;

// Keep the _start name
#[no_mangle]
// ! Means its not allowed to return
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    cat_os::exit_qemu(cat_os::QemuExitCode::Failed);

    loop {
        x86_64::instructions::hlt()
    }
}

// Called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    cat_os::exit_qemu(cat_os::QemuExitCode::Success);
    cat_os::test_panic_handler(info);
}

fn should_fail() {
    serial_println!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

// TODO: Fix, this file wont run when doing cargo test?
