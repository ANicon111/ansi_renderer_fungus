use crate::{
    color::Color,
    colors::Colors,
    pixel::{Pixel, Pixels},
};

pub struct Borders;

impl Borders {
    pub const EMPTY: Border = Border {
        top_left: Pixels::EMPTY,
        top: Pixels::EMPTY,
        top_right: Pixels::EMPTY,
        right: Pixels::EMPTY,
        bottom_right: Pixels::EMPTY,
        bottom: Pixels::EMPTY,
        bottom_left: Pixels::EMPTY,
        left: Pixels::EMPTY,
    };
    pub const SIMPLE: Border = Border {
        top_left: Pixel {
            value: '┌',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
        top: Pixel {
            value: '─',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
        top_right: Pixel {
            value: '┐',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
        right: Pixel {
            value: '│',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
        bottom_right: Pixel {
            value: '┘',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
        bottom: Pixel {
            value: '─',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
        bottom_left: Pixel {
            value: '└',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
        left: Pixel {
            value: '│',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
    };
    pub const DOUBLE: Border = Border {
        top_left: Pixel {
            value: '╔',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
        top: Pixel {
            value: '═',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
        top_right: Pixel {
            value: '╗',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
        right: Pixel {
            value: '║',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
        bottom_right: Pixel {
            value: '╝',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
        bottom: Pixel {
            value: '═',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
        bottom_left: Pixel {
            value: '╚',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
        left: Pixel {
            value: '║',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
    };
    pub const ROUNDED: Border = Border {
        top_left: Pixel {
            value: '╭',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
        top: Pixel {
            value: '─',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
        top_right: Pixel {
            value: '╮',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
        right: Pixel {
            value: '│',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
        bottom_right: Pixel {
            value: '╯',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
        bottom: Pixel {
            value: '─',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
        bottom_left: Pixel {
            value: '╰',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
        left: Pixel {
            value: '│',
            background: Colors::TRANSPARENT,
            foreground: Colors::WHITE,
        },
    };
}

#[derive(Clone, Debug, Copy)]
pub struct Border {
    pub top_left: Pixel,
    pub top: Pixel,
    pub top_right: Pixel,
    pub right: Pixel,
    pub bottom_right: Pixel,
    pub bottom: Pixel,
    pub bottom_left: Pixel,
    pub left: Pixel,
}

impl Border {
    pub fn from_pixels(
        top_left: Pixel,
        top: Pixel,
        top_right: Pixel,
        right: Pixel,
        bottom_right: Pixel,
        bottom: Pixel,
        bottom_left: Pixel,
        left: Pixel,
    ) -> Border {
        Border {
            top_left,
            top,
            top_right,
            right,
            bottom_right,
            bottom,
            bottom_left,
            left,
        }
    }

    pub fn from_chars(
        top_left: char,
        top: char,
        top_right: char,
        right: char,
        bottom_right: char,
        bottom: char,
        bottom_left: char,
        left: char,
    ) -> Border {
        Border {
            top_left: Pixel {
                value: top_left,
                background: Colors::TRANSPARENT,
                foreground: Colors::WHITE,
            },
            top: Pixel {
                value: top,
                background: Colors::TRANSPARENT,
                foreground: Colors::WHITE,
            },
            top_right: Pixel {
                value: top_right,
                background: Colors::TRANSPARENT,
                foreground: Colors::WHITE,
            },
            right: Pixel {
                value: right,
                background: Colors::TRANSPARENT,
                foreground: Colors::WHITE,
            },
            bottom_right: Pixel {
                value: bottom_right,
                background: Colors::TRANSPARENT,
                foreground: Colors::WHITE,
            },
            bottom: Pixel {
                value: bottom,
                background: Colors::TRANSPARENT,
                foreground: Colors::WHITE,
            },
            bottom_left: Pixel {
                value: bottom_left,
                background: Colors::TRANSPARENT,
                foreground: Colors::WHITE,
            },
            left: Pixel {
                value: left,
                background: Colors::TRANSPARENT,
                foreground: Colors::WHITE,
            },
        }
    }

    pub fn set_background_all(&mut self, color: Color) -> &mut Self {
        self.top_left.background = color;
        self.top.background = color;
        self.top_right.background = color;
        self.right.background = color;
        self.bottom_right.background = color;
        self.bottom.background = color;
        self.bottom_left.background = color;
        self.left.background = color;
        self
    }

    pub fn set_foreground_all(&mut self, color: Color) -> &mut Self {
        self.top_left.foreground = color;
        self.top.foreground = color;
        self.top_right.foreground = color;
        self.right.foreground = color;
        self.bottom_right.foreground = color;
        self.bottom.foreground = color;
        self.bottom_left.foreground = color;
        self.left.foreground = color;
        self
    }
}
