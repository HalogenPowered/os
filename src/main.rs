#![feature(allocator_api)]
#![no_std]
#![no_main]

//mod allocator;
mod logging;

use bootloader::boot_info::Optional;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use raw_cpuid::CpuId;
use crate::logging::printk_init;

entry_point!(main);

fn main(boot_info: &'static mut BootInfo) -> ! {
    println!("Starting Halogen OS version 0.1.0.");
    write_cpu_info();
    write_memory_info(boot_info);
    loop {}
}

fn terminal_init(info: &'static mut BootInfo) {
    let framebuffer = match &mut info.framebuffer {
        Optional::Some(value) => value,
        Optional::None => return,
    };
    let info = framebuffer.info().clone();
    printk_init(framebuffer.buffer_mut(), info);
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

fn write_memory_info(boot_info: &BootInfo) {
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

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
