use bootloader::boot_info::FrameBufferInfo;
use conquer_once::spin::OnceCell;
use log::LevelFilter;
use printk::LockedPrintk;

static PRINTK: OnceCell<LockedPrintk> = OnceCell::uninit();

pub fn printk_init(buffer: &'static mut [u8], info: FrameBufferInfo) {
    let kernel_logger = PRINTK.get_or_init(move || LockedPrintk::new(buffer, info));
    log::set_logger(kernel_logger).expect("Logger already set!");
    log::set_max_level(LevelFilter::Trace);
    log::info!()
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
