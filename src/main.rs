// Dont link standard library
#![no_std]
// Dont use normal entry point
#![no_main]

use bootloader::{entry_point, BootInfo};
use cat_os::{
    clear,
    memory::{self, translate_addr},
    println,
};
use core::panic::PanicInfo;
use x86_64::{
    structures::paging::{PageTable, Translate},
    VirtAddr,
};

// Set entry point
entry_point!(kernel_main);

// ! Means its not allowed to return
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("You should'nt see this");
    clear!();

    println!("                  Loading CatOS                   ");
    println!("                    By Cyteon                     ");
    println!("      License: GNU General Public License 3.0     ");
    println!("          https://github.com/cyteon/catos         ");
    println!("\nBooting System...");

    cat_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    // Initialize mapper
    let mapper = unsafe { memory::init(phys_mem_offset) };

    println!("Testing Memory Pages");
    let addresses = [
        // VGA BUFFER
        0xb8000,
        boot_info.physical_memory_offset,
    ];

    for &addr in &addresses {
        let virt = VirtAddr::new(addr);
        let phys = mapper.translate_addr(virt);

        println!("{:?} -> {:?}", virt, phys)
    }

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
