use core::slice;
use bootloader::boot_info::FrameBuffer;
use conquer_once::spin::OnceCell;
use log::LevelFilter;
use printk::LockedPrintk;

static PRINTK: OnceCell<LockedPrintk> = OnceCell::uninit();

pub fn printk_init(buffer: &'static FrameBuffer) {
    let kernel_logger = PRINTK.get_or_init(move || LockedPrintk::new(buf_to_mut(buffer.buffer()), buffer.info()));
    log::set_logger(kernel_logger).expect("Logger already set!");
    log::set_max_level(LevelFilter::Trace);
}

// This is really, really unsafe, and is really not an example to follow,
// but it works fine, and gets around Rust's borrowing rules that prohibit us from
// passing a mutable FrameBuffer to the init function.
fn buf_to_mut(buffer: &[u8]) -> &mut [u8] {
    unsafe { slice::from_raw_parts_mut(buffer.as_ptr() as *mut u8, buffer.len()) }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (log::info!($($arg)*))
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!($($arg)*))
}
