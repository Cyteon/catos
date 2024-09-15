// Dont link standard library
#![no_std]
// Dont use normal entry point
#![no_main]

use bootloader::{entry_point, BootInfo};
use cat_os::{memory::active_level_4_table, println};
use core::panic::PanicInfo;
use x86_64::{structures::paging::PageTable, VirtAddr};

// Set entry point
entry_point!(kernel_main);

// ! Means its not allowed to return
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("                  Loading CatOS                   ");
    println!("                    By Cyteon                     ");
    println!("      License: GNU General Public License 3.0     ");
    println!("          https://github.com/cyteon/catos         ");
    println!("\nBooting System...");

    cat_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);

            let phys = entry.frame().unwrap().start_address();
            let virt = phys.as_u64() + boot_info.physical_memory_offset;
            let ptr = VirtAddr::new(virt).as_mut_ptr();

            let l3_table: &PageTable = unsafe { &*ptr };

            for (i, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() {
                    println!("L3 Entry {}: {:?}", i, entry);
                }
            }
        }
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
