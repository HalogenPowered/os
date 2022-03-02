#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use halogen_os::{exit_qemu, serial_print, serial_println, QemuExitCode};
use lazy_static::lazy_static;
use volatile::Volatile;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

entry_point!(stack_overflow);

fn stack_overflow(_: &'static mut BootInfo) -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");
    halogen_os::gdt::init();
    init_test_idt();
    stack_overflow_test();
    panic!("Execution continued after stack overflow!");
}

#[allow(unconditional_recursion)]
fn stack_overflow_test() {
    stack_overflow_test();
    Volatile::new(&0).read();
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(halogen_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(_: InterruptStackFrame, _: u64) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success)
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    halogen_os::test_panic_handler(info)
}
