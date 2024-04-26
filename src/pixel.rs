use std::fmt::Display;

use crate::{color::Color, colors::Colors};

pub struct Pixels;

impl Pixels {
    pub const EMPTY: Pixel = Pixel {
        value: '\0',
        background: Colors::INVALID,
        foreground: Colors::INVALID,
    };

    pub const DEFAULT: Pixel = Pixel {
        value: ' ',
        background: Colors::BLACK,
        foreground: Colors::WHITE,
    };
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Pixel {
    pub value: char,
    pub background: Color,
    pub foreground: Color,
}

impl Pixel {
    pub fn new() -> Pixel {
        Pixels::EMPTY
    }

    pub fn with_overlay(&self, overlay: &Pixel) -> Pixel {
        if "▀▄▅▆▇█▉▊▋▌▐▙▛▜▟▚▞▓▒".contains(self.value) && overlay.value > ' '
        {
            return Pixel {
                value: overlay.value,
                background: self.foreground.with_overlay(overlay.background),
                foreground: self.foreground.with_overlay(overlay.foreground),
            };
        } else if overlay.value > ' ' {
            return Pixel {
                value: overlay.value,
                background: self.background.with_overlay(overlay.background),
                foreground: self.background.with_overlay(overlay.foreground),
            };
        } else if overlay.value == ' ' {
            return Pixel {
                value: self.value,
                background: self.background.with_overlay(overlay.background),
                foreground: self.foreground.with_overlay(overlay.background),
            };
        } else {
            return (*self).clone();
        }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\x1B[48;2;{};{};{}m\x1B[38;2;{};{};{}m{}\x1B[39m\x1B[49m",
            self.background.red,
            self.background.green,
            self.background.blue,
            self.foreground.red,
            self.foreground.green,
            self.foreground.blue,
            self.value,
        )
    }
}
