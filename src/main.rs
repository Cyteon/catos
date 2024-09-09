

// Dont link standard library
#![no_std]
// Not use normal entry point
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

static HELLO: &[u8] = b"Hello, World! (why am i making my own OS???)";

// No mangle so no random function name stuff
#[no_mangle]
// ! Means its not allowed to return
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}

// Called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
