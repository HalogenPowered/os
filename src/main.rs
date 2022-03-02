#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(halogen_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use halogen_os::{allocator, halt_loop, init, init_headless, println};
use halogen_os::memory::{self, BootInfoFrameAllocator};
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    #[cfg(test)]
    run_tests();

    init(boot_info);
    println!("Starting Halogen OS version 0.1.0.");

    // Setup heap memory so we can perform heap allocations
    setup_heap_memory(boot_info);

    println!("It did not crash!");
    halt_loop()
}

#[cfg(test)]
fn run_tests() -> ! {
    init_headless();
    test_main();
    halt_loop()
}

fn setup_heap_memory(boot_info: &'static BootInfo) {
    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
    let mut mapper = unsafe { memory::init(physical_memory_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::new(&boot_info.memory_regions) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialization failed!");
}

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}", info);
    halt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    halogen_os::test_panic_handler(info)
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn trivial_assertion() {
        assert_eq!(1, 1);
    }
}
