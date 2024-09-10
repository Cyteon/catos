// Dont link standard library
#![no_std]
// Dont use normal entry point
#![no_main]

use cat_os::println;
use core::panic::PanicInfo;

// No mangle so no random function name stuff
#[no_mangle]
// ! Means its not allowed to return
pub extern "C" fn _start() -> ! {
    println!("Hello, World!");

    cat_os::init();

    println!("I didnt crash :D");

    loop {
        x86_64::instructions::hlt()
    }
}

// Called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {
        x86_64::instructions::hlt();
    }
}
