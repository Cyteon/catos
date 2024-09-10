// Dont use standard lib
#![no_std]
// Enable x86-interrupt
#![feature(abi_x86_interrupt)]

pub mod gdt;
pub mod interrupts;
pub mod vga_buffer;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}