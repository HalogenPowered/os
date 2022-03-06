use core::fmt::{Arguments, Write};
use core::slice;
use bootloader::boot_info::FrameBuffer;
use conquer_once::spin::OnceCell;
use log::LevelFilter;
use printk::Printk;
use spin::{Mutex, Once};
use x86_64::instructions::interrupts;

static PRINTK: Once<Mutex<Printk>> = Once::new();

pub fn init(buffer: &'static FrameBuffer) {
    PRINTK.call_once(|| Mutex::new(Printk::new(buf_to_mut(buffer.buffer()), buffer.info())));
}

// This is really, really unsafe, and is really not an example to follow,
// but it works fine, and gets around Rust's borrowing rules that prohibit us from
// passing a mutable FrameBuffer to the init function.
fn buf_to_mut(buffer: &[u8]) -> &mut [u8] {
    unsafe { slice::from_raw_parts_mut(buffer.as_ptr() as *mut u8, buffer.len()) }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::logging::_print(format_args!($($arg)*)))
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn _print(args: Arguments) {
    interrupts::without_interrupts(|| {
        PRINTK.get().unwrap().lock().write_fmt(args).expect("Printing to logger failed!");
    })
}
