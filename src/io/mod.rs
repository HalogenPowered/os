mod logging;
pub mod serial;

use bootloader::boot_info::Optional;
use bootloader::BootInfo;

#[cfg(not(test))]
pub fn init(boot_info: &'static BootInfo) {
    let framebuffer = match &boot_info.framebuffer {
        Optional::Some(value) => value,
        Optional::None => return,
    };
    logging::printk_init(framebuffer);
}
