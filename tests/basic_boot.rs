// Dont link standard library
#![no_std]
// Dont use normal entry point
#![no_main]
// Testing stuff
#![feature(custom_test_frameworks)]
#![test_runner(cat_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use cat_os::println;
use core::panic::PanicInfo;

// Keep the _start name
#[no_mangle]
// ! Means its not allowed to return
pub extern "C" fn _start() -> ! {
    test_main();

    loop {
        x86_64::instructions::hlt()
    }
}

// Called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cat_os::test_panic_handler(info);
}

#[test_case]
fn test_println() {
    println!("Println simple test");
}

// TODO: Fix, this file wont run when doing cargo test?
