// Dont use standard lib
#![no_std]
// Enable x86-interrupt
#![feature(abi_x86_interrupt)]

pub mod interrupts;
pub mod vga_buffer;

pub fn init() {
    interrupts::init_idt();
}
