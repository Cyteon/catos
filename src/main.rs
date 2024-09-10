// Dont link standard library
#![no_std]
// Dont use normal entry point
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

// No mangle so no random function name stuff
#[no_mangle]
// ! Means its not allowed to return
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;

    println!("Hello, World!");
    write!(
        vga_buffer::WRITER.lock(),
        "Here is some stuff for you: {} <= {} == {}\n",
        1,
        -0.5,
        false
    )
    .unwrap();

    loop {}
}

// Called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}
