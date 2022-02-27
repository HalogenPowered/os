#![no_std]
#![no_main]

mod lateinit;
mod tty;

use crate::tty::{init_terminal, terminal};
use bootloader::boot_info::{FrameBuffer, Optional};
use bootloader::{entry_point, BootInfo};
use core::fmt::Write;
use core::panic::PanicInfo;
use raw_cpuid::CpuId;
use tty::Terminal;

entry_point!(main);

fn main(boot_info: &'static mut BootInfo) -> ! {
    terminal_init(boot_info);
    sys_write("Starting Halogen OS version 0.1.0.");
    write_cpu_info();
    sys_write("Initializing terminal");
    loop {}
}

fn terminal_init(info: &mut BootInfo) {
    let framebuffer = match &mut info.framebuffer {
        Optional::Some(value) => value,
        Optional::None => return,
    };
    let info = framebuffer.info().clone();
    init_terminal(framebuffer.buffer(), info)
}

fn write_cpu_info() {
    let cpuid = CpuId::new();
    sys_write("Processor Details:");
    if let Some(brand) = cpuid.get_processor_brand_string() {
        write!(terminal(), "* Brand: ");
        writeln!(terminal(), "{}", brand.as_str());
    }
    if let Some(vendor) = cpuid.get_vendor_info() {
        write!(terminal(), "* Vendor: ");
        writeln!(terminal(), "{}", vendor);
    };
    if let Some(info) = cpuid.get_feature_info() {
        write!(terminal(), "* Family: ");
        writeln!(terminal(), "{}", info.family_id());
        write!(terminal(), "* Model: ");
        writeln!(terminal(), "{}", info.model_id());
    };
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

fn sys_write(message: &str) {
    writeln!(terminal(), "{}", message);
}
