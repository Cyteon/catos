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

    // Test interrupts
    x86_64::instructions::interrupts::int3();

    println!("Here is some stuff for you: {} <= {} == {}", 1, -0.5, false);

    loop {}
}

// Called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}
