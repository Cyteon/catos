use bootloader::BootInfo;

use crate::{print, println};

use x86_64::structures::paging::Translate;

pub fn test(boot_info: &'static BootInfo) {
    use crate::memory;
    use x86_64::{structures::paging::Page, VirtAddr}; // new import

    print!("Testing memory mapper\n\n");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();

    unsafe { page_ptr.offset(480).write_volatile(0x_f021_f069_f048u64) };
    print!("\n\n");

    // Memory pages

    println!("Testing memory pages");

    // Initialize mapper
    let mapper = unsafe { memory::init(phys_mem_offset) };
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
}
