// Dont link standard library
#![no_std]
// Dont use normal entry point
#![no_main]

use bootloader::{entry_point, BootInfo};
use cat_os::{clear, println, test};
use core::panic::PanicInfo;

// Set entry point
entry_point!(kernel_main);

// ! Means its not allowed to return
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use cat_os::memory::BootInfoFrameAllocator;

    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    println!("You should not see this");
    clear!();

    println!("                  Loading CatOS                   ");
    println!("                    By Cyteon                     ");
    println!("      License: GNU General Public License 3.0     ");
    println!("          https://github.com/cyteon/catos         ");
    println!("\nBooting System...");

    cat_os::init();

    test::test(boot_info);

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
