// Dont link standard library
#![no_std]
// Dont use normal entry point
#![no_main]
// Testing stuff
#![feature(custom_test_frameworks)]
#![test_runner(cat_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use cat_os::{clear, println, serial_print, serial_println};
use core::panic::PanicInfo;

// Set entry point
entry_point!(kernel_main);

// ! Means its not allowed to return
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use cat_os::memory::BootInfoFrameAllocator;
    use cat_os::{allocator, memory};
    use x86_64::VirtAddr;

    println!("You should not see this");
    clear!();

    println!("                  Loading CatOS                   ");
    println!("                    By Cyteon                     ");
    println!("      License: GNU General Public License 3.0     ");
    println!("          https://github.com/cyteon/catos         ");
    println!("\nBooting System...");

    cat_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    loop {
        x86_64::instructions::hlt()
    }
}

// Called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {
        x86_64::instructions::hlt();
    }
}

// TODO: Fix
// I have to have this here and in lib.rs, if i remove from here or other it complains
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cat_os::test_panic_handler(info)
}

// TODO: Fix, this wont run
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
