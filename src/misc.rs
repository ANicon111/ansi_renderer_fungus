use std::io::{self, Write};

use crate::{
    colors::Colors,
    geometry::Dimension,
    pixel::{Pixel, Pixels},
};

pub(crate) fn process_percent(mut percent_double: &str) -> Result<f64, &str> {
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

pub(crate) fn generic_dimension_calc(
    dim: &Dimension,
    parent_width: i64,
    parent_height: i64,
    renderer_width: i64,
    renderer_height: i64,
    horizontal: bool,
) -> i64 {
    match dim {
        Dimension::Auto => 0,
        Dimension::Pixel(val) => *val,
        Dimension::Percent(val) => (if horizontal {
            parent_width
        } else {
            parent_height
        } as f64
            * val
            * 0.01)
            .round() as i64,
        Dimension::PW(val) => (parent_width as f64 * val * 0.01).round() as i64,
        Dimension::PH(val) => (parent_height as f64 * val * 0.01).round() as i64,
        Dimension::PMin(val) => {
            (parent_width.min(parent_height) as f64 * val * 0.01).round() as i64
        }
        Dimension::PMax(val) => {
            (parent_width.max(parent_height) as f64 * val * 0.01).round() as i64
        }
        Dimension::VW(val) => (renderer_width as f64 * val * 0.01).round() as i64,
        Dimension::VH(val) => (renderer_height as f64 * val * 0.01).round() as i64,
        Dimension::VMin(val) => {
            (renderer_width.min(renderer_height) as f64 * val * 0.01).round() as i64
        }
        Dimension::VMax(val) => {
            (renderer_width.max(renderer_height) as f64 * val * 0.01).round() as i64
        }
    }
}

pub(crate) struct BufferedConsole {
    buffer: Vec<u8>,
    last_pixel: Pixel,
}

impl BufferedConsole {
    pub(crate) fn new() -> BufferedConsole {
        BufferedConsole {
            buffer: Vec::with_capacity(0xffff),
            last_pixel: Pixel {
                value: '\0',
                background: Colors::INVALID,
                foreground: Colors::INVALID,
            },
        }
    }

    pub(crate) fn print(&mut self, mut pixel: Pixel) {
        if pixel.value == '\0' {
            pixel = Pixels::DEFAULT;
        }

        if pixel.value >= ' ' && pixel.background.valid && pixel.foreground.valid {
            if self.last_pixel.background != pixel.background {
                for c in format!(
                    "\x1b[48;2;{};{};{}m",
                    pixel.background.red, pixel.background.green, pixel.background.blue
                )
                .as_bytes()
                {
                    self.buffer.push(*c);
                }
            }
            if self.last_pixel.foreground != pixel.foreground {
                for c in format!(
                    "\x1b[38;2;{};{};{}m",
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

    pub(crate) fn set_cursor_position(&mut self, x: i64, y: i64) {
        for c in format!("\x1b[{};{}H", y + 1, x + 1).as_bytes() {
            self.buffer.push(*c);
        }
    }

    pub(crate) fn flush(&mut self) {
        for c in "\x1b[39m\x1b[49m".as_bytes() {
            self.buffer.push(*c);
        }
        {
            let mut stdout = io::stdout().lock();
            stdout.write_all(&self.buffer).unwrap();
            stdout.flush().unwrap();
        }
        self.buffer.clear();
    }

    pub(crate) fn clear(&mut self) {
        self.buffer.clear();
    }
}
