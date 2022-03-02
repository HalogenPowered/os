#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(halogen_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use halogen_os::println;

entry_point!(basic_boot);

fn basic_boot(_: &'static mut BootInfo) -> ! {
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    halogen_os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
