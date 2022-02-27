#![feature(allocator_api)]
#![feature(alloc_error_handler)]
#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

extern crate alloc;

mod allocator;
mod logging;
mod memory;
mod interrupt;
mod gdt;

use bootloader::boot_info::Optional;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use raw_cpuid::CpuId;
use x86_64::VirtAddr;
use crate::logging::printk_init;
use crate::memory::BootInfoFrameAllocator;

entry_point!(main);

fn main(boot_info: &'static mut BootInfo) -> ! {
    init(boot_info);
    println!("Starting Halogen OS version 0.1.0.");

    // Setup heap memory so we can perform heap allocations
    //setup_heap_memory(boot_info);

    //println!("It did not crash!");
    halt_loop()
}

fn init(boot_info: &'static BootInfo) {
    terminal_init(boot_info);
    gdt::init();
    interrupt::init_idt();
    unsafe { interrupt::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

fn terminal_init(info: &'static BootInfo) {
    let framebuffer = match &info.framebuffer {
        Optional::Some(value) => value,
        Optional::None => return,
    };
    printk_init(framebuffer);
}

fn setup_heap_memory(boot_info: &'static BootInfo) {
    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
    let mut mapper = unsafe { memory::init(physical_memory_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::new(&boot_info.memory_regions) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialization failed!");
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}", info);
    halt_loop();
}

fn halt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
