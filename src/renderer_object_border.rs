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

#[derive(Clone)]
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
    pub fn new(
        tl: char,
        t: char,
        tr: char,
        r: char,
        br: char,
        b: char,
        bl: char,
        l: char,
    ) -> Border {
        Border {
            top_left: Pixel {
                value: tl,
                background: Colors::TRANSPARENT,
                foreground: Colors::WHITE,
            },
            top: Pixel {
                value: t,
                background: Colors::TRANSPARENT,
                foreground: Colors::WHITE,
            },
            top_right: Pixel {
                value: tr,
                background: Colors::TRANSPARENT,
                foreground: Colors::WHITE,
            },
            right: Pixel {
                value: r,
                background: Colors::TRANSPARENT,
                foreground: Colors::WHITE,
            },
            bottom_right: Pixel {
                value: br,
                background: Colors::TRANSPARENT,
                foreground: Colors::WHITE,
            },
            bottom: Pixel {
                value: b,
                background: Colors::TRANSPARENT,
                foreground: Colors::WHITE,
            },
            bottom_left: Pixel {
                value: bl,
                background: Colors::TRANSPARENT,
                foreground: Colors::WHITE,
            },
            left: Pixel {
                value: l,
                background: Colors::TRANSPARENT,
                foreground: Colors::WHITE,
            },
        }
    }

    pub fn set_background(&mut self, color: Color) {
        self.top_left.background = color;
        self.top.background = color;
        self.top_right.background = color;
        self.right.background = color;
        self.bottom_right.background = color;
        self.bottom.background = color;
        self.bottom_left.background = color;
        self.left.background = color;
    }

    pub fn set_foreground(&mut self, color: Color) {
        self.top_left.foreground = color;
        self.top.foreground = color;
        self.top_right.foreground = color;
        self.right.foreground = color;
        self.bottom_right.foreground = color;
        self.bottom.foreground = color;
        self.bottom_left.foreground = color;
        self.left.foreground = color;
    }
}
