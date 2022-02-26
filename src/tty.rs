use core::fmt::{Arguments, Write};
use core::{ptr, slice};
use bootloader::boot_info::{FrameBufferInfo, PixelFormat};
use noto_sans_mono_bitmap::{get_bitmap, get_bitmap_width, BitmapChar, BitmapHeight, FontWeight};
use crate::lateinit::LateInit;

const LINE_SPACING: usize = 0;
const LOG_SPACING: usize = 2;

static TERMINAL: LateInit<Terminal> = LateInit::new();

pub fn terminal() -> &'static mut Terminal {
    TERMINAL.get_mut()
}

pub fn init_terminal(buffer: &[u8], info: FrameBufferInfo) {
    TERMINAL.init(Terminal::new(buffer, info))
}

pub struct Terminal {
    buffer_start: *mut u8,
    buffer_length: usize,
    info: FrameBufferInfo,
    x_position: usize,
    y_position: usize
}

impl Terminal {
    fn new(buffer: &[u8], info: FrameBufferInfo) -> Self {
        let mut logger = Self {
            buffer_start: buffer.as_ptr() as *mut u8,
            buffer_length: buffer.len(),
            info,
            x_position: 0,
            y_position: 0
        };
        logger.clear();
        logger
    }

    fn newline(&mut self) {
        self.y_position += 14 + LINE_SPACING;
        self.carriage_return()
    }

    fn add_vspace(&mut self, space: usize) {
        self.y_position += space;
    }

    fn carriage_return(&mut self) {
        self.x_position = 0;
    }

    pub fn clear(&mut self) {
        self.x_position = 0;
        self.y_position = 0;
        self.as_slice().fill(0);
    }

    fn width(&self) -> usize {
        self.info.horizontal_resolution
    }

    fn height(&self) -> usize {
        self.info.vertical_resolution
    }

    fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            other => {
                if self.x_position >= self.width() {
                    self.newline();
                }
                const BITMAP_LETTER_WIDTH: usize = get_bitmap_width(FontWeight::Regular, BitmapHeight::Size14);
                if self.y_position >= (self.height() - BITMAP_LETTER_WIDTH) {
                    self.clear();
                }
                let bitmap_char = get_bitmap(other, FontWeight::Regular, BitmapHeight::Size14).unwrap();
                self.write_rendered_char(bitmap_char);
            }
        }
    }

    fn write_rendered_char(&mut self, rendered_char: BitmapChar) {
        for (y, row) in rendered_char.bitmap().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.write_pixel(self.x_position + x, self.y_position + y, *byte);
            }
        }
        self.x_position += rendered_char.width();
    }

    fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        let pixel_offset = y * self.info.stride + x;
        let color = match self.info.pixel_format {
            PixelFormat::RGB => [intensity, intensity, intensity / 2, 0],
            PixelFormat::BGR => [intensity / 2, intensity, intensity, 0],
            PixelFormat::U8 => [if intensity > 200 { 0xf } else { 0 }, 0, 0, 0],
            _ => return
        };
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;
        let slice = self.as_slice();
        slice[byte_offset..(byte_offset + bytes_per_pixel)].copy_from_slice(&color[..bytes_per_pixel]);
        let _ = unsafe { ptr::read_volatile(&slice[byte_offset]) };
    }

    fn as_slice(&self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.buffer_start, self.buffer_length) }
    }
}

impl Write for Terminal {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for character in s.chars() {
            self.write_char(character);
        }
        Ok(())
    }
}
