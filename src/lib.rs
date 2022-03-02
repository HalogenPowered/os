#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::alloc::Layout;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

pub mod allocator;
pub mod memory;
pub mod interrupt;
pub mod gdt;
pub mod io;

pub fn init(boot_info: &'static BootInfo) {
    io::init(boot_info);
    init_headless();
}

pub fn init_headless() {
    gdt::init();
    interrupt::init_idt();
    unsafe { interrupt::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T: Fn()> Testable for T {
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) -> ! {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success)
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
    halt_loop();
}

pub fn halt_loop() -> ! {
    loop { x86_64::instructions::hlt(); }
}

#[cfg(test)]
entry_point!(kernel_test);

#[cfg(test)]
fn kernel_test(boot_info: &'static mut BootInfo) -> ! {
    init_headless();
    test_main();
    halt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("Allocation error! {:?}", layout)
}
