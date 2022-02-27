#![feature(allocator_api)]
#![feature(alloc_error_handler)]
#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

extern crate alloc;

mod allocator;
mod logging;
mod memory;
mod interrupts;

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use bootloader::boot_info::Optional;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use raw_cpuid::CpuId;
use x86_64::VirtAddr;
use crate::logging::printk_init;
use crate::memory::BootInfoFrameAllocator;

entry_point!(main);

fn main(boot_info: &'static mut BootInfo) -> ! {
    // Initialize the terminal so we can actually write things to the screen
    terminal_init(boot_info);
    println!("Starting Halogen OS version 0.1.0.");

    // Print information about the currently running CPU (from CPUID)
    //write_cpu_info();
    // Print information about the available memory regions given to us by the bootloader
    //write_memory_info(boot_info);
    // Setup heap memory so we can perform heap allocations
    setup_heap_memory(boot_info);

    println!("It did not crash!");
    halt_loop()
}

fn terminal_init(info: &'static BootInfo) {
    let framebuffer = match &info.framebuffer {
        Optional::Some(value) => value,
        Optional::None => return,
    };
    printk_init(framebuffer);
}

fn write_cpu_info() {
    let cpuid = CpuId::new();
    println!("Processor Details:");
    if let Some(brand) = cpuid.get_processor_brand_string() {
        println!("* Brand: {}", brand.as_str());
    }
    if let Some(vendor) = cpuid.get_vendor_info() {
        println!("* Vendor: {}", vendor);
    };
    if let Some(info) = cpuid.get_feature_info() {
        println!("* Family: {}", info.family_id());
        println!("* Model: {}", info.model_id());
    };
}

fn write_memory_info(boot_info: &'static BootInfo) {
    println!("Memory regions length: {}", boot_info.memory_regions.len());
    let mut total_memory = 0;
    for i in 0..boot_info.memory_regions.len() - 2 {
        let region = boot_info.memory_regions[i];
        println!("Memory region #{}:", i);
        println!("* Start: {} (hex: 0x{:x})", region.start, region.start);
        println!("* End: {} (hex: 0x{:x})", region.end, region.end);
        println!("* Size: {} (bytes: {}, KB: {})",
                 region.end - region.start,
                 (region.end - region.start) * 8,
                 ((region.end - region.start) * 8) / 1024
        );
        println!("* Kind: {:?}", region.kind);
        total_memory += (region.end - region.start) * 8;
    }
    println!("Total memory (bytes): {}", total_memory);
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
