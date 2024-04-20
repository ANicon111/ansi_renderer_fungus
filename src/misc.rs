use std::io::{self, Write};

use crate::{
    colors::Colors,
    pixel::{Pixel, Pixels},
};

pub fn process_percent(mut percent_double: &str) -> Result<f64, &str> {
    percent_double = percent_double.trim();
    let alpha_mod = if percent_double.ends_with('%') {
        percent_double = percent_double[0..percent_double.len() - 1].trim();
        0.01
    } else {
        1.0
    };
    match percent_double.parse::<f64>() {
        Ok(val) => Ok(val * alpha_mod),
        Err(_) => Err("Couldn't parse percent"),
    }
}

pub struct BufferedConsole {
    buffer: Vec<u8>,
    last_pixel: Pixel,
}

impl BufferedConsole {
    pub fn new() -> BufferedConsole {
        BufferedConsole {
            buffer: Vec::with_capacity(0xffff),
            last_pixel: Pixel {
                value: '\0',
                background: Colors::INVALID,
                foreground: Colors::INVALID,
            },
        }
    }

    pub fn print(&mut self, mut pixel: Pixel) {
        if pixel.value == '\0' {
            pixel = Pixels::DEFAULT;
        }

        if pixel.value >= ' ' && pixel.background.valid && pixel.foreground.valid {
            if self.last_pixel.background != pixel.background {
                for c in format!(
                    "\x1B[48;2;{};{};{}m",
                    pixel.background.red, pixel.background.green, pixel.background.blue
                )
                .as_bytes()
                {
                    self.buffer.push(*c);
                }
            }
            if self.last_pixel.foreground != pixel.foreground {
                for c in format!(
                    "\x1B[38;2;{};{};{}m",
                    pixel.background.red, pixel.background.green, pixel.background.blue
                )
                .as_bytes()
                {
                    self.buffer.push(*c);
                }
            }
            for c in pixel.to_string().as_bytes() {
                self.buffer.push(*c);
            }
            self.last_pixel = pixel;
            return;
        }
    }

    pub fn set_cursor_position(&mut self, x: i64, y: i64) {
        for c in format!("\x1B[{};{}H", y + 1, x + 1).as_bytes() {
            self.buffer.push(*c);
        }
    }

    pub fn flush(&mut self) {
        for c in "\x1B[39m\x1B[49m".as_bytes() {
            self.buffer.push(*c);
        }
        {
            let mut stdout = io::stdout().lock();
            stdout.write_all(&self.buffer).unwrap();
        }
        self.buffer.clear();
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}
