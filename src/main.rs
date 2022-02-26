#![no_std]
#![no_main]

extern crate alloc;

mod cpuid;
mod lateinit;
mod tty;

use crate::tty::{init_terminal, terminal};
use bootloader::boot_info::{FrameBuffer, Optional};
use bootloader::{entry_point, BootInfo};
use core::fmt::Write;
use core::panic::PanicInfo;
use tty::Terminal;

entry_point!(main);

fn main(boot_info: &'static mut BootInfo) -> ! {
    terminal_init(boot_info);
    sys_write("Starting Halogen OS version 0.1.0.");
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

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

fn sys_write(message: &str) {
    writeln!(terminal(), "{}", message);
}
