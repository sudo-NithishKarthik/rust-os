#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

pub mod gdt;
pub mod interrupts;
pub mod vga_buffer;

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn init() {
    interrupts::init_idt();
    gdt::init();
    unsafe { interrupts::PICS.lock().initialize() };
    // the CPU by default does not listen to hardware interrupts, we enable it to do so here
    x86_64::instructions::interrupts::enable();
}
