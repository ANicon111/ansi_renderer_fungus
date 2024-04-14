use std::fmt::Display;

use crate::{
    geometry::Dimension,
    misc::process_percent,
    renderer_object_style::{AlignmentX, AlignmentY},
};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ColorLayer {
    Background,
    Foreground,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct ColorArea {
    pub color: Color,
    pub layer: ColorLayer,
    pub x: Dimension,
    pub y: Dimension,
    pub width: Dimension,
    pub height: Dimension,
    pub external_alignment_x: Option<AlignmentX>,
    pub external_alignment_y: Option<AlignmentY>,
    pub renderer_object_index: usize,
}

impl ColorArea {
    pub fn new(color: Color, layer: ColorLayer) -> ColorArea {
        ColorArea {
            color,
            layer,
            x: Dimension::Auto,
            y: Dimension::Auto,
            width: Dimension::Auto,
            height: Dimension::Auto,
            external_alignment_x: None,
            external_alignment_y: None,
            renderer_object_index: 0,
        }
    }
    pub fn with_geometry(
        color: Color,
        layer: ColorLayer,
        (x, y, width, height): (Dimension, Dimension, Dimension, Dimension),
    ) -> ColorArea {
        ColorArea {
            color,
            layer,
            x,
            y,
            width,
            height,
            external_alignment_x: None,
            external_alignment_y: None,
            renderer_object_index: 0,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
    pub valid: bool,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_name: &str = self.get_name();
        match (!color_name.is_empty(), self.alpha == 255) {
            (true, true) => write!(
                f,
                "({} - red:{}, green:{}, blue:{})",
                color_name, self.red, self.green, self.blue
            ),
            (true, false) => write!(
                f,
                "({} - red:{}, green:{}, blue:{}, alpha:{})",
                color_name, self.red, self.green, self.blue, self.alpha
            ),
            (false, true) => write!(
                f,
                "(red:{}, green:{}, blue:{})",
                self.red, self.green, self.blue
            ),
            (false, false) => write!(
                f,
                "(red:{}, green:{}, blue:{}, alpha:{})",
                self.red, self.green, self.blue, self.alpha
            ),
        }
    }
}

impl Color {
    pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: f32) -> Color {
        Color {
            red,
            green,
            blue,
            alpha: (alpha.clamp(0.0, 1.0) * 255.0).round() as u8,
            valid: true,
        }
    }

    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Color {
        Color::from_rgba(red, green, blue, 1.0)
    }

    pub fn with_red(&self, red: u8) -> Color {
        Color::from_rgba(red, self.green, self.blue, (self.alpha as f32) / 255.0)
    }

    pub fn with_green(&self, green: u8) -> Color {
        Color::from_rgba(self.red, green, self.blue, (self.alpha as f32) / 255.0)
    }

    pub fn with_blue(&self, blue: u8) -> Color {
        Color::from_rgba(self.red, self.green, blue, (self.alpha as f32) / 255.0)
    }

    pub fn with_alpha(&self, alpha: f32) -> Color {
        Color::from_rgba(self.red, self.green, self.blue, alpha)
    }

    pub fn from_u32(val: u32) -> Color {
        Color {
            red: (val / 0x1000000) as u8,
            green: (val / 0x10000 % 0x100) as u8,
            blue: (val / 0x100 % 0x100) as u8,
            alpha: (val % 0x100) as u8,
            valid: true,
        }
    }
    pub fn from_u16(val: u16) -> Color {
        Color {
            red: (val / 0x1000 * 0x11) as u8,
            green: (val / 0x100 % 0x10 * 0x11) as u8,
            blue: (val / 0x10 % 0x10 * 0x11) as u8,
            alpha: (val % 0x10 * 0x11) as u8,
            valid: true,
        }
    }

    pub fn from_hsla(hue: f32, saturation: f32, luminosity: f32, alpha: f32) -> Color {
        let h: f32 = if hue < 0.0 {
            360.0 + hue % 360.0
        } else {
            hue % 360.0
        };

        fn hue_calc(h: f32) -> f32 {
            ((3.0 - h % 360.0 / 60.0).abs() - 1.0).clamp(0.0, 1.0)
        }

        let s: f32 = saturation.clamp(0.0, 1.0);
        let l: f32 = luminosity.clamp(0.0, 1.0);

        let c: f32 = s * (1.0 - (1.0 - 2.0 * luminosity).abs());
        let b: f32 = l - c / 2.0;

        let r: u8 = (255.0 * (b + c * hue_calc(h + 0.0)))
            .round()
            .clamp(0.0, 255.0) as u8;
        let g: u8 = (255.0 * (b + c * hue_calc(h + 240.0)))
            .round()
            .clamp(0.0, 255.0) as u8;
        let b: u8 = (255.0 * (b + c * hue_calc(h + 120.0)))
            .round()
            .clamp(0.0, 255.0) as u8;
        let a: u8 = (alpha.clamp(0.0, 1.0) * 255.0).round() as u8;

        Color {
            red: r,
            green: g,
            blue: b,
            alpha: a,
            valid: true,
        }
    }

    pub fn from_hsl(hue: f32, saturation: f32, luminosity: f32) -> Color {
        Color::from_hsla(hue, saturation, luminosity, 1.0)
    }

    pub fn hue(&self) -> f32 {
        let red: f32 = self.red as f32 / 255.0;
        let green: f32 = self.green as f32 / 255.0;
        let blue: f32 = self.blue as f32 / 255.0;

        let max: f32 = red.max(blue.max(green));
        let min: f32 = red.min(blue.min(green));

        if max == min {
            return 0.0;
        };

        match (red == max, green == max, blue == max) {
            (true, _, _) => 60.0 * (green - blue) / (max - min),
            (_, true, _) => 60.0 * (2.0 + (blue - red) / (max - min)),
            (_, _, true) => 60.0 * (4.0 + (red - green) / (max - min)),
            _ => 420.69, //this should not be possible
        }
    }

    pub fn saturation(&self) -> f32 {
        let red: f32 = self.red as f32 / 255.0;
        let green: f32 = self.green as f32 / 255.0;
        let blue: f32 = self.blue as f32 / 255.0;

        let max: f32 = red.max(blue.max(green));
        let min: f32 = red.min(blue.min(green));

        if max == min {
            return 0.0;
        };
        return (max - min) / (1.0 - (2.0 * self.luminosity() - 1.0).abs());
    }

    pub fn luminosity(&self) -> f32 {
        let red: f32 = self.red as f32 / 255.0;
        let green: f32 = self.green as f32 / 255.0;
        let blue: f32 = self.blue as f32 / 255.0;

        let max: f32 = red.max(blue.max(green));
        let min: f32 = red.min(blue.min(green));

        return (max + min) / 2.0;
    }

    pub fn with_hue(&self, hue: f32) -> Color {
        Color::from_hsla(
            hue,
            self.saturation(),
            self.luminosity(),
            (self.alpha as f32) / 255.0,
        )
    }

    pub fn with_saturation(&self, saturation: f32) -> Color {
        Color::from_hsla(
            self.hue(),
            saturation,
            self.luminosity(),
            (self.alpha as f32) / 255.0,
        )
    }

    pub fn with_luminosity(&self, luminosity: f32) -> Color {
        Color::from_hsla(
            self.hue(),
            self.saturation(),
            luminosity,
            (self.alpha as f32) / 255.0,
        )
    }

    pub fn with_overlay(self, overlay: Color) -> Color {
        match (self, overlay) {
            (Colors::INVALID, value) => value,
            (value, Colors::INVALID) => value,
            (under, over) => Color {
                red: ((over.red as f32 * over.alpha as f32
                    + under.red as f32 * (255 - over.alpha) as f32)
                    / 255.0)
                    .round() as u8,
                green: ((over.green as f32 * over.alpha as f32
                    + under.green as f32 * (255 - over.alpha) as f32)
                    / 255.0)
                    .round() as u8,
                blue: ((over.blue as f32 * over.alpha as f32
                    + under.blue as f32 * (255 - over.alpha) as f32)
                    / 255.0)
                    .round() as u8,
                alpha: (over.alpha as f32 + under.alpha as f32 * (255 - over.alpha) as f32 / 255.0)
                    as u8,
                valid: true,
            },
        }
    }

    pub fn inverted(&self) -> Color {
        Color {
            red: 255 - self.red,
            green: 255 - self.green,
            blue: 255 - self.blue,
            alpha: self.alpha,
            valid: self.valid,
        }
    }

    // here begin the definitions, feel free to fold them

    #[allow(unreachable_patterns)]
    pub fn get_name(&self) -> &str {
        if !self.valid {
            return "INVALID";
        }
        if self.alpha == 0 {
            return "TRANSPARENT";
        }
        let color: Color = Color {
            red: self.red,
            green: self.green,
            blue: self.blue,
            alpha: 255,
            valid: true,
        };

        match color {
            Colors::ALICE_BLUE => "ALICE_BLUE",
            Colors::ANTIQUE_WHITE => "ANTIQUE_WHITE",
            Colors::AQUA => "AQUA",
            Colors::AQUAMARINE => "AQUAMARINE",
            Colors::AZURE => "AZURE",
            Colors::BEIGE => "BEIGE",
            Colors::BISQUE => "BISQUE",
            Colors::BLACK => "BLACK",
            Colors::BLANCHED_ALMOND => "BLANCHED_ALMOND",
            Colors::BLUE => "BLUE",
            Colors::BLUE_VIOLET => "BLUE_VIOLET",
            Colors::BROWN => "BROWN",
            Colors::BURLY_WOOD => "BURLY_WOOD",
            Colors::CADET_BLUE => "CADET_BLUE",
            Colors::CHARTREUSE => "CHARTREUSE",
            Colors::CHOCOLATE => "CHOCOLATE",
            Colors::CORAL => "CORAL",
            Colors::CORNFLOWER_BLUE => "CORNFLOWER_BLUE",
            Colors::CORN_SILK => "CORN_SILK",
            Colors::CRIMSON => "CRIMSON",
            Colors::CYAN => "CYAN",
            Colors::DARK_BLUE => "DARK_BLUE",
            Colors::DARK_CYAN => "DARK_CYAN",
            Colors::DARK_GOLDENROD => "DARK_GOLDENROD",
            Colors::DARK_GRAY => "DARK_GRAY",
            Colors::DARK_GREEN => "DARK_GREEN",
            Colors::DARK_GREY => "DARK_GREY",
            Colors::DARK_KHAKI => "DARK_KHAKI",
            Colors::DARK_MAGENTA => "DARK_MAGENTA",
            Colors::DARK_OLIVE_GREEN => "DARK_OLIVE_GREEN",
            Colors::DARK_ORANGE => "DARK_ORANGE",
            Colors::DARK_ORCHID => "DARK_ORCHID",
            Colors::DARK_RED => "DARK_RED",
            Colors::DARK_SALMON => "DARK_SALMON",
            Colors::DARK_SEA_GREEN => "DARK_SEA_GREEN",
            Colors::DARK_SLATE_BLUE => "DARK_SLATE_BLUE",
            Colors::DARK_SLATE_GRAY => "DARK_SLATE_GRAY",
            Colors::DARK_SLATE_GREY => "DARK_SLATE_GREY",
            Colors::DARK_TURQUOISE => "DARK_TURQUOISE",
            Colors::DARK_VIOLET => "DARK_VIOLET",
            Colors::DEEP_PINK => "DEEP_PINK",
            Colors::DEEP_SKY_BLUE => "DEEP_SKY_BLUE",
            Colors::DIM_GRAY => "DIM_GRAY",
            Colors::DIM_GREY => "DIM_GREY",
            Colors::DODGER_BLUE => "DODGER_BLUE",
            Colors::FIREBRICK => "FIREBRICK",
            Colors::FLORAL_WHITE => "FLORAL_WHITE",
            Colors::FOREST_GREEN => "FOREST_GREEN",
            Colors::FUCHSIA => "FUCHSIA",
            Colors::GAINSBORO => "GAINSBORO",
            Colors::GHOST_WHITE => "GHOST_WHITE",
            Colors::GOLDENROD => "GOLDENROD",
            Colors::GOLD => "GOLD",
            Colors::GRAY => "GRAY",
            Colors::GREEN => "GREEN",
            Colors::GREEN_YELLOW => "GREEN_YELLOW",
            Colors::GREY => "GREY",
            Colors::HONEYDEW => "HONEYDEW",
            Colors::HOT_PINK => "HOT_PINK",
            Colors::INDIAN_RED => "INDIAN_RED",
            Colors::INDIGO => "INDIGO",
            Colors::IVORY => "IVORY",
            Colors::KHAKI => "KHAKI",
            Colors::LAVENDER_BLUSH => "LAVENDER_BLUSH",
            Colors::LAVENDER => "LAVENDER",
            Colors::LAWN_GREEN => "LAWN_GREEN",
            Colors::LEMON_CHIFFON => "LEMON_CHIFFON",
            Colors::LIGHT_BLUE => "LIGHT_BLUE",
            Colors::LIGHT_CORAL => "LIGHT_CORAL",
            Colors::LIGHT_CYAN => "LIGHT_CYAN",
            Colors::LIGHT_GOLDENROD_YELLOW => "LIGHT_GOLDENROD_YELLOW",
            Colors::LIGHT_GRAY => "LIGHT_GRAY",
            Colors::LIGHT_GREEN => "LIGHT_GREEN",
            Colors::LIGHT_GREY => "LIGHT_GREY",
            Colors::LIGHT_PINK => "LIGHT_PINK",
            Colors::LIGHT_SALMON => "LIGHT_SALMON",
            Colors::LIGHT_SEA_GREEN => "LIGHT_SEA_GREEN",
            Colors::LIGHT_SKY_BLUE => "LIGHT_SKY_BLUE",
            Colors::LIGHT_SLATE_GRAY => "LIGHT_SLATE_GRAY",
            Colors::LIGHT_SLATE_GREY => "LIGHT_SLATE_GREY",
            Colors::LIGHT_STEEL_BLUE => "LIGHT_STEEL_BLUE",
            Colors::LIGHT_YELLOW => "LIGHT_YELLOW",
            Colors::LIME => "LIME",
            Colors::LIME_GREEN => "LIME_GREEN",
            Colors::LINEN => "LINEN",
            Colors::MAGENTA => "MAGENTA",
            Colors::MAROON => "MAROON",
            Colors::MEDIUM_AQUAMARINE => "MEDIUM_AQUAMARINE",
            Colors::MEDIUM_BLUE => "MEDIUM_BLUE",
            Colors::MEDIUM_ORCHID => "MEDIUM_ORCHID",
            Colors::MEDIUM_PURPLE => "MEDIUM_PURPLE",
            Colors::MEDIUM_SEA_GREEN => "MEDIUM_SEA_GREEN",
            Colors::MEDIUM_SLATE_BLUE => "MEDIUM_SLATE_BLUE",
            Colors::MEDIUM_SPRING_GREEN => "MEDIUM_SPRING_GREEN",
            Colors::MEDIUM_TURQUOISE => "MEDIUM_TURQUOISE",
            Colors::MEDIUM_VIOLET_RED => "MEDIUM_VIOLET_RED",
            Colors::MIDNIGHT_BLUE => "MIDNIGHT_BLUE",
            Colors::MINT_CREAM => "MINT_CREAM",
            Colors::MISTY_ROSE => "MISTY_ROSE",
            Colors::MOCCASIN => "MOCCASIN",
            Colors::NAVAJO_WHITE => "NAVAJO_WHITE",
            Colors::NAVY => "NAVY",
            Colors::OLD_LACE => "OLD_LACE",
            Colors::OLIVE => "OLIVE",
            Colors::OLIVE_DRAB => "OLIVE_DRAB",
            Colors::ORANGE => "ORANGE",
            Colors::ORANGE_RED => "ORANGE_RED",
            Colors::ORCHID => "ORCHID",
            Colors::PALE_GOLDENROD => "PALE_GOLDENROD",
            Colors::PALE_GREEN => "PALE_GREEN",
            Colors::PALE_TURQUOISE => "PALE_TURQUOISE",
            Colors::PALE_VIOLET_RED => "PALE_VIOLET_RED",
            Colors::PAPAYA_WHIP => "PAPAYA_WHIP",
            Colors::PEACH_PUFF => "PEACH_PUFF",
            Colors::PERU => "PERU",
            Colors::PINK => "PINK",
            Colors::PLUM => "PLUM",
            Colors::POWDER_BLUE => "POWDER_BLUE",
            Colors::PURPLE => "PURPLE",
            Colors::REBECCA_PURPLE => "REBECCA_PURPLE",
            Colors::RED => "RED",
            Colors::ROSY_BROWN => "ROSY_BROWN",
            Colors::ROYAL_BLUE => "ROYAL_BLUE",
            Colors::SADDLE_BROWN => "SADDLE_BROWN",
            Colors::SALMON => "SALMON",
            Colors::SANDY_BROWN => "SANDY_BROWN",
            Colors::SEA_GREEN => "SEA_GREEN",
            Colors::SEASHELL => "SEASHELL",
            Colors::SIENNA => "SIENNA",
            Colors::SILVER => "SILVER",
            Colors::SKY_BLUE => "SKY_BLUE",
            Colors::SLATE_BLUE => "SLATE_BLUE",
            Colors::SLATE_GRAY => "SLATE_GRAY",
            Colors::SLATE_GREY => "SLATE_GREY",
            Colors::SNOW => "SNOW",
            Colors::SPRING_GREEN => "SPRING_GREEN",
            Colors::STEEL_BLUE => "STEEL_BLUE",
            Colors::TAN => "TAN",
            Colors::TEAL => "TEAL",
            Colors::THISTLE => "THISTLE",
            Colors::TOMATO => "TOMATO",
            Colors::TURQUOISE => "TURQUOISE",
            Colors::VIOLET => "VIOLET",
            Colors::WHEAT => "WHEAT",
            Colors::WHITE => "WHITE",
            Colors::WHITE_SMOKE => "WHITE_SMOKE",
            Colors::YELLOW_GREEN => "YELLOW_GREEN",
            Colors::YELLOW => "YELLOW",
            _ => "",
        }
    }

    /* spellchecker: disable */
    pub fn from_name(name: &str) -> Result<Color, &str> {
        match name.to_lowercase().replace("_", "").trim() {
            "transparent" => Ok(Colors::TRANSPARENT),
            "aliceblue" => Ok(Colors::ALICE_BLUE),
            "antiquewhite" => Ok(Colors::ANTIQUE_WHITE),
            "aqua" => Ok(Colors::AQUA),
            "aquamarine" => Ok(Colors::AQUAMARINE),
            "azure" => Ok(Colors::AZURE),
            "beige" => Ok(Colors::BEIGE),
            "bisque" => Ok(Colors::BISQUE),
            "black" => Ok(Colors::BLACK),
            "blanchedalmond" => Ok(Colors::BLANCHED_ALMOND),
            "blue" => Ok(Colors::BLUE),
            "blueviolet" => Ok(Colors::BLUE_VIOLET),
            "brown" => Ok(Colors::BROWN),
            "burlywood" => Ok(Colors::BURLY_WOOD),
            "cadetblue" => Ok(Colors::CADET_BLUE),
            "chartreuse" => Ok(Colors::CHARTREUSE),
            "chocolate" => Ok(Colors::CHOCOLATE),
            "coral" => Ok(Colors::CORAL),
            "cornflower_blue" => Ok(Colors::CORNFLOWER_BLUE),
            "cornsilk" => Ok(Colors::CORN_SILK),
            "crimson" => Ok(Colors::CRIMSON),
            "cyan" => Ok(Colors::CYAN),
            "darkblue" => Ok(Colors::DARK_BLUE),
            "darkcyan" => Ok(Colors::DARK_CYAN),
            "darkgoldenrod" => Ok(Colors::DARK_GOLDENROD),
            "darkgray" => Ok(Colors::DARK_GRAY),
            "darkgreen" => Ok(Colors::DARK_GREEN),
            "darkgrey" => Ok(Colors::DARK_GREY),
            "darkkhaki" => Ok(Colors::DARK_KHAKI),
            "darkmagenta" => Ok(Colors::DARK_MAGENTA),
            "darkolive_green" => Ok(Colors::DARK_OLIVE_GREEN),
            "darkorange" => Ok(Colors::DARK_ORANGE),
            "darkorchid" => Ok(Colors::DARK_ORCHID),
            "darkred" => Ok(Colors::DARK_RED),
            "darksalmon" => Ok(Colors::DARK_SALMON),
            "darksea_green" => Ok(Colors::DARK_SEA_GREEN),
            "darkslate_blue" => Ok(Colors::DARK_SLATE_BLUE),
            "darkslate_gray" => Ok(Colors::DARK_SLATE_GRAY),
            "darkslate_grey" => Ok(Colors::DARK_SLATE_GREY),
            "darkturquoise" => Ok(Colors::DARK_TURQUOISE),
            "darkviolet" => Ok(Colors::DARK_VIOLET),
            "deeppink" => Ok(Colors::DEEP_PINK),
            "deepsky_blue" => Ok(Colors::DEEP_SKY_BLUE),
            "dimgray" => Ok(Colors::DIM_GRAY),
            "dimgrey" => Ok(Colors::DIM_GREY),
            "dodgerblue" => Ok(Colors::DODGER_BLUE),
            "firebrick" => Ok(Colors::FIREBRICK),
            "floralwhite" => Ok(Colors::FLORAL_WHITE),
            "forestgreen" => Ok(Colors::FOREST_GREEN),
            "fuchsia" => Ok(Colors::FUCHSIA),
            "gainsboro" => Ok(Colors::GAINSBORO),
            "ghostwhite" => Ok(Colors::GHOST_WHITE),
            "goldenrod" => Ok(Colors::GOLDENROD),
            "gold" => Ok(Colors::GOLD),
            "gray" => Ok(Colors::GRAY),
            "green" => Ok(Colors::GREEN),
            "greenyellow" => Ok(Colors::GREEN_YELLOW),
            "grey" => Ok(Colors::GREY),
            "honeydew" => Ok(Colors::HONEYDEW),
            "hotpink" => Ok(Colors::HOT_PINK),
            "indianred" => Ok(Colors::INDIAN_RED),
            "indigo" => Ok(Colors::INDIGO),
            "ivory" => Ok(Colors::IVORY),
            "khaki" => Ok(Colors::KHAKI),
            "lavenderblush" => Ok(Colors::LAVENDER_BLUSH),
            "lavender" => Ok(Colors::LAVENDER),
            "lawngreen" => Ok(Colors::LAWN_GREEN),
            "lemonchiffon" => Ok(Colors::LEMON_CHIFFON),
            "lightblue" => Ok(Colors::LIGHT_BLUE),
            "lightcoral" => Ok(Colors::LIGHT_CORAL),
            "lightcyan" => Ok(Colors::LIGHT_CYAN),
            "lightgoldenrod_yellow" => Ok(Colors::LIGHT_GOLDENROD_YELLOW),
            "lightgray" => Ok(Colors::LIGHT_GRAY),
            "lightgreen" => Ok(Colors::LIGHT_GREEN),
            "lightgrey" => Ok(Colors::LIGHT_GREY),
            "lightpink" => Ok(Colors::LIGHT_PINK),
            "lightsalmon" => Ok(Colors::LIGHT_SALMON),
            "lightsea_green" => Ok(Colors::LIGHT_SEA_GREEN),
            "lightsky_blue" => Ok(Colors::LIGHT_SKY_BLUE),
            "lightslate_gray" => Ok(Colors::LIGHT_SLATE_GRAY),
            "lightslate_grey" => Ok(Colors::LIGHT_SLATE_GREY),
            "lightsteel_blue" => Ok(Colors::LIGHT_STEEL_BLUE),
            "lightyellow" => Ok(Colors::LIGHT_YELLOW),
            "lime" => Ok(Colors::LIME),
            "limegreen" => Ok(Colors::LIME_GREEN),
            "linen" => Ok(Colors::LINEN),
            "magenta" => Ok(Colors::MAGENTA),
            "maroon" => Ok(Colors::MAROON),
            "mediumaquamarine" => Ok(Colors::MEDIUM_AQUAMARINE),
            "mediumblue" => Ok(Colors::MEDIUM_BLUE),
            "mediumorchid" => Ok(Colors::MEDIUM_ORCHID),
            "mediumpurple" => Ok(Colors::MEDIUM_PURPLE),
            "mediumsea_green" => Ok(Colors::MEDIUM_SEA_GREEN),
            "mediumslate_blue" => Ok(Colors::MEDIUM_SLATE_BLUE),
            "mediumspring_green" => Ok(Colors::MEDIUM_SPRING_GREEN),
            "mediumturquoise" => Ok(Colors::MEDIUM_TURQUOISE),
            "mediumviolet_red" => Ok(Colors::MEDIUM_VIOLET_RED),
            "midnightblue" => Ok(Colors::MIDNIGHT_BLUE),
            "mintcream" => Ok(Colors::MINT_CREAM),
            "mistyrose" => Ok(Colors::MISTY_ROSE),
            "moccasin" => Ok(Colors::MOCCASIN),
            "navajowhite" => Ok(Colors::NAVAJO_WHITE),
            "navy" => Ok(Colors::NAVY),
            "oldlace" => Ok(Colors::OLD_LACE),
            "olive" => Ok(Colors::OLIVE),
            "olivedrab" => Ok(Colors::OLIVE_DRAB),
            "orange" => Ok(Colors::ORANGE),
            "orangered" => Ok(Colors::ORANGE_RED),
            "orchid" => Ok(Colors::ORCHID),
            "palegoldenrod" => Ok(Colors::PALE_GOLDENROD),
            "palegreen" => Ok(Colors::PALE_GREEN),
            "paleturquoise" => Ok(Colors::PALE_TURQUOISE),
            "paleviolet_red" => Ok(Colors::PALE_VIOLET_RED),
            "papayawhip" => Ok(Colors::PAPAYA_WHIP),
            "peachpuff" => Ok(Colors::PEACH_PUFF),
            "peru" => Ok(Colors::PERU),
            "pink" => Ok(Colors::PINK),
            "plum" => Ok(Colors::PLUM),
            "powderblue" => Ok(Colors::POWDER_BLUE),
            "purple" => Ok(Colors::PURPLE),
            "rebeccapurple" => Ok(Colors::REBECCA_PURPLE),
            "red" => Ok(Colors::RED),
            "rosybrown" => Ok(Colors::ROSY_BROWN),
            "royalblue" => Ok(Colors::ROYAL_BLUE),
            "saddlebrown" => Ok(Colors::SADDLE_BROWN),
            "salmon" => Ok(Colors::SALMON),
            "sandybrown" => Ok(Colors::SANDY_BROWN),
            "seagreen" => Ok(Colors::SEA_GREEN),
            "seashell" => Ok(Colors::SEASHELL),
            "sienna" => Ok(Colors::SIENNA),
            "silver" => Ok(Colors::SILVER),
            "skyblue" => Ok(Colors::SKY_BLUE),
            "slateblue" => Ok(Colors::SLATE_BLUE),
            "slategray" => Ok(Colors::SLATE_GRAY),
            "slategrey" => Ok(Colors::SLATE_GREY),
            "snow" => Ok(Colors::SNOW),
            "springgreen" => Ok(Colors::SPRING_GREEN),
            "steelblue" => Ok(Colors::STEEL_BLUE),
            "tan" => Ok(Colors::TAN),
            "teal" => Ok(Colors::TEAL),
            "thistle" => Ok(Colors::THISTLE),
            "tomato" => Ok(Colors::TOMATO),
            "turquoise" => Ok(Colors::TURQUOISE),
            "violet" => Ok(Colors::VIOLET),
            "wheat" => Ok(Colors::WHEAT),
            "white" => Ok(Colors::WHITE),
            "whitesmoke" => Ok(Colors::WHITE_SMOKE),
            "yellowgreen" => Ok(Colors::YELLOW_GREEN),
            "yellow" => Ok(Colors::YELLOW),
            _ => Err("Couldn't parse color"),
        }
    }
    /* spellchecker: enable */

    pub fn from_html(mut html: &str) -> Result<Color, &str> {
        html = html.trim();

        let from_name = Color::from_name(html);
        if from_name.is_ok() {
            return Ok(from_name.unwrap());
        };

        if html.starts_with('#') {
            html = &html[1..].trim();
            let length = html.len();
            return match length {
                8 => match u32::from_str_radix(html, 16) {
                    Ok(val) => Ok(Color::from_u32(val)),
                    Err(_) => Err("Couldn't parse #RRGGBBAA format"),
                },
                6 => match u32::from_str_radix(html, 16) {
                    Ok(val) => Ok(Color::from_u32(val * 0x100 + 0xff)),
                    Err(_) => Err("Couldn't parse #RRGGBB format"),
                },
                4 => match u16::from_str_radix(html, 16) {
                    Ok(val) => Ok(Color::from_u16(val)),
                    Err(_) => Err("Couldn't parse #RGBA format"),
                },
                3 => match u16::from_str_radix(html, 16) {
                    Ok(val) => Ok(Color::from_u16(val * 0x10 + 0xf)),
                    Err(_) => Err("Couldn't parse #RGB format"),
                },
                _ => Err("Invalid color format. Supported hexadecimal formats are #RRGGBBAA, #RRGGBB, #RGBA and #RGB."),
            };
        }

        if html.starts_with("rgba") {
            html = &html[4..].trim();
            let rgba: Vec<&str>;

            if html.starts_with('(') && html.ends_with(')') {
                rgba = html[1..html.len() - 1].trim().split(',').collect();
            } else {
                return Err("Invalid color format. The correct rgba(u8, u8, u8, f32) function structure is rgba(r, g, b, a). Spaces don't matter.");
            }

            if rgba.len() != 4 {
                return Err("Invalid color format. The correct rgba(u8, u8, u8, f32) function structure is rgba(r, g, b, a). Spaces don't matter.");
            }

            return match (
                rgba[0].trim().parse::<u8>(),
                rgba[1].trim().parse::<u8>(),
                rgba[2].trim().parse::<u8>(),
                process_percent(rgba[3])
            ) {
                (Ok(red),Ok(green),Ok(blue),Ok(alpha))=>
                    Ok(Color::from_rgba(red, green, blue, alpha)),

                _=>Err("Invalid color format. The correct rgba(u8, u8, u8, f32) function structure is rgba(r, g, b, a). Spaces don't matter")
            };
        }

        if html.starts_with("rgb") {
            html = &html[3..].trim();
            let rgba: Vec<&str>;

            if html.starts_with('(') && html.ends_with(')') {
                rgba = html[1..html.len() - 1].trim().split(',').collect();
            } else {
                return Err("Invalid color format. The correct rgb(u8, u8, u8) function structure is rgb(r, g, b). Spaces don't matter.");
            }

            if rgba.len() != 3 {
                return Err("Invalid color format. The correct rgb(u8, u8, u8) function structure is rgb(r, g, b). Spaces don't matter.");
            }

            return match (
                rgba[0].trim().parse::<u8>(),
                rgba[1].trim().parse::<u8>(),
                rgba[2].trim().parse::<u8>()
            ) {
                (Ok(red),Ok(green),Ok(blue))=>
                    Ok(Color::from_rgb(red, green, blue)),

                _=>Err("Invalid color format. The correct rgb(u8, u8, u8) function structure is rgb(r, g, b). Spaces don't matter")
            };
        }

        if html.starts_with("hsla") {
            html = &html[4..].trim();
            let rgba: Vec<&str>;

            if html.starts_with('(') && html.ends_with(')') {
                rgba = html[1..html.len() - 1].trim().split(',').collect();
            } else {
                return Err("Invalid color format. The correct hsla(f32, f32, f32, f32) function structure is hsla(h, s, l, a). Spaces don't matter.");
            }

            if rgba.len() != 4 {
                return Err("Invalid color format. The correct hsla(f32, f32, f32, f32) function structure is hsla(h, s, l, a). Spaces don't matter.");
            }

            return match (
                rgba[0].trim().parse::<f32>(),
                process_percent(rgba[1]),
                process_percent(rgba[2]),
                process_percent(rgba[3])
            ) {
                (Ok(hue),Ok(saturation),Ok(luminosity),Ok(alpha))=>
                    Ok(Color::from_hsla(hue, saturation, luminosity, alpha)),

                _=>Err("Invalid color format. The correct hsla(f32, f32, f32, f32) function structure is hsla(h, s, l, a). Spaces don't matter.")
            };
        }

        if html.starts_with("hsl") {
            html = &html[3..].trim();
            let rgba: Vec<&str>;

            if html.starts_with('(') && html.ends_with(')') {
                rgba = html[1..html.len() - 1].trim().split(',').collect();
            } else {
                return Err("Invalid color format. The correct hsl(f32, f32, f32) function structure is hsl(h, s, l). Spaces don't matter.");
            }

            if rgba.len() != 3 {
                return Err("Invalid color format. The correct hsl(f32, f32, f32) function structure is hsl(h, s, l). Spaces don't matter.");
            }

            return match (
                rgba[0].trim().parse::<f32>(),
                process_percent(rgba[1]),
                process_percent(rgba[2])
            ) {
                (Ok(hue),Ok(saturation),Ok(luminosity))=>
                    Ok(Color::from_hsl(hue, saturation, luminosity)),

                _=>Err("Invalid color format. The correct hsl(f32, f32, f32) function structure is hsl(h, s, l). Spaces don't matter.")
            };
        }
        Err("Invalid color name or format. Make sure to use supported HTML color names or formats.")
    }
}

pub struct Colors;

impl Colors {
    pub const INVALID: Color = Color {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 0,
        valid: false,
    };

    pub const TRANSPARENT: Color = Color {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 0,
        valid: true,
    };
    pub const ALICE_BLUE: Color = Color {
        red: 240,
        green: 248,
        blue: 255,
        alpha: 255,
        valid: true,
    };
    pub const ANTIQUE_WHITE: Color = Color {
        red: 250,
        green: 235,
        blue: 215,
        alpha: 255,
        valid: true,
    };
    pub const AQUA: Color = Color {
        red: 0,
        green: 255,
        blue: 255,
        alpha: 255,
        valid: true,
    };
    pub const AQUAMARINE: Color = Color {
        red: 127,
        green: 255,
        blue: 212,
        alpha: 255,
        valid: true,
    };
    pub const AZURE: Color = Color {
        red: 240,
        green: 255,
        blue: 255,
        alpha: 255,
        valid: true,
    };
    pub const BEIGE: Color = Color {
        red: 245,
        green: 245,
        blue: 220,
        alpha: 255,
        valid: true,
    };
    pub const BISQUE: Color = Color {
        red: 255,
        green: 228,
        blue: 196,
        alpha: 255,
        valid: true,
    };
    pub const BLACK: Color = Color {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 255,
        valid: true,
    };
    pub const BLANCHED_ALMOND: Color = Color {
        red: 255,
        green: 235,
        blue: 205,
        alpha: 255,
        valid: true,
    };
    pub const BLUE: Color = Color {
        red: 0,
        green: 0,
        blue: 255,
        alpha: 255,
        valid: true,
    };
    pub const BLUE_VIOLET: Color = Color {
        red: 138,
        green: 43,
        blue: 226,
        alpha: 255,
        valid: true,
    };
    pub const BROWN: Color = Color {
        red: 165,
        green: 42,
        blue: 42,
        alpha: 255,
        valid: true,
    };
    pub const BURLY_WOOD: Color = Color {
        red: 222,
        green: 184,
        blue: 135,
        alpha: 255,
        valid: true,
    };
    pub const CADET_BLUE: Color = Color {
        red: 95,
        green: 158,
        blue: 160,
        alpha: 255,
        valid: true,
    };
    pub const CHARTREUSE: Color = Color {
        red: 127,
        green: 255,
        blue: 0,
        alpha: 255,
        valid: true,
    };
    pub const CHOCOLATE: Color = Color {
        red: 210,
        green: 105,
        blue: 30,
        alpha: 255,
        valid: true,
    };
    pub const CORAL: Color = Color {
        red: 255,
        green: 127,
        blue: 80,
        alpha: 255,
        valid: true,
    };
    pub const CORNFLOWER_BLUE: Color = Color {
        red: 100,
        green: 149,
        blue: 237,
        alpha: 255,
        valid: true,
    };
    pub const CORN_SILK: Color = Color {
        red: 255,
        green: 248,
        blue: 220,
        alpha: 255,
        valid: true,
    };
    pub const CRIMSON: Color = Color {
        red: 220,
        green: 20,
        blue: 60,
        alpha: 255,
        valid: true,
    };
    pub const CYAN: Color = Color {
        red: 0,
        green: 255,
        blue: 255,
        alpha: 255,
        valid: true,
    };
    pub const DARK_BLUE: Color = Color {
        red: 0,
        green: 0,
        blue: 139,
        alpha: 255,
        valid: true,
    };
    pub const DARK_CYAN: Color = Color {
        red: 0,
        green: 139,
        blue: 139,
        alpha: 255,
        valid: true,
    };
    pub const DARK_GOLDENROD: Color = Color {
        red: 184,
        green: 134,
        blue: 11,
        alpha: 255,
        valid: true,
    };
    pub const DARK_GRAY: Color = Color {
        red: 169,
        green: 169,
        blue: 169,
        alpha: 255,
        valid: true,
    };
    pub const DARK_GREEN: Color = Color {
        red: 0,
        green: 100,
        blue: 0,
        alpha: 255,
        valid: true,
    };
    pub const DARK_GREY: Color = Color {
        red: 169,
        green: 169,
        blue: 169,
        alpha: 255,
        valid: true,
    };
    pub const DARK_KHAKI: Color = Color {
        red: 189,
        green: 183,
        blue: 107,
        alpha: 255,
        valid: true,
    };
    pub const DARK_MAGENTA: Color = Color {
        red: 139,
        green: 0,
        blue: 139,
        alpha: 255,
        valid: true,
    };
    pub const DARK_OLIVE_GREEN: Color = Color {
        red: 85,
        green: 107,
        blue: 47,
        alpha: 255,
        valid: true,
    };
    pub const DARK_ORANGE: Color = Color {
        red: 255,
        green: 140,
        blue: 0,
        alpha: 255,
        valid: true,
    };
    pub const DARK_ORCHID: Color = Color {
        red: 153,
        green: 50,
        blue: 204,
        alpha: 255,
        valid: true,
    };
    pub const DARK_RED: Color = Color {
        red: 139,
        green: 0,
        blue: 0,
        alpha: 255,
        valid: true,
    };
    pub const DARK_SALMON: Color = Color {
        red: 233,
        green: 150,
        blue: 122,
        alpha: 255,
        valid: true,
    };
    pub const DARK_SEA_GREEN: Color = Color {
        red: 143,
        green: 188,
        blue: 143,
        alpha: 255,
        valid: true,
    };
    pub const DARK_SLATE_BLUE: Color = Color {
        red: 72,
        green: 61,
        blue: 139,
        alpha: 255,
        valid: true,
    };
    pub const DARK_SLATE_GRAY: Color = Color {
        red: 47,
        green: 79,
        blue: 79,
        alpha: 255,
        valid: true,
    };
    pub const DARK_SLATE_GREY: Color = Color {
        red: 47,
        green: 79,
        blue: 79,
        alpha: 255,
        valid: true,
    };
    pub const DARK_TURQUOISE: Color = Color {
        red: 0,
        green: 206,
        blue: 209,
        alpha: 255,
        valid: true,
    };
    pub const DARK_VIOLET: Color = Color {
        red: 148,
        green: 0,
        blue: 211,
        alpha: 255,
        valid: true,
    };
    pub const DEEP_PINK: Color = Color {
        red: 255,
        green: 20,
        blue: 147,
        alpha: 255,
        valid: true,
    };
    pub const DEEP_SKY_BLUE: Color = Color {
        red: 0,
        green: 191,
        blue: 255,
        alpha: 255,
        valid: true,
    };
    pub const DIM_GRAY: Color = Color {
        red: 105,
        green: 105,
        blue: 105,
        alpha: 255,
        valid: true,
    };
    pub const DIM_GREY: Color = Color {
        red: 105,
        green: 105,
        blue: 105,
        alpha: 255,
        valid: true,
    };
    pub const DODGER_BLUE: Color = Color {
        red: 30,
        green: 144,
        blue: 255,
        alpha: 255,
        valid: true,
    };
    pub const FIREBRICK: Color = Color {
        red: 178,
        green: 34,
        blue: 34,
        alpha: 255,
        valid: true,
    };
    pub const FLORAL_WHITE: Color = Color {
        red: 255,
        green: 250,
        blue: 240,
        alpha: 255,
        valid: true,
    };
    pub const FOREST_GREEN: Color = Color {
        red: 34,
        green: 139,
        blue: 34,
        alpha: 255,
        valid: true,
    };
    pub const FUCHSIA: Color = Color {
        red: 255,
        green: 0,
        blue: 255,
        alpha: 255,
        valid: true,
    };
    pub const GAINSBORO: Color = Color {
        red: 220,
        green: 220,
        blue: 220,
        alpha: 255,
        valid: true,
    };
    pub const GHOST_WHITE: Color = Color {
        red: 248,
        green: 248,
        blue: 255,
        alpha: 255,
        valid: true,
    };
    pub const GOLDENROD: Color = Color {
        red: 218,
        green: 165,
        blue: 32,
        alpha: 255,
        valid: true,
    };
    pub const GOLD: Color = Color {
        red: 255,
        green: 215,
        blue: 0,
        alpha: 255,
        valid: true,
    };
    pub const GRAY: Color = Color {
        red: 128,
        green: 128,
        blue: 128,
        alpha: 255,
        valid: true,
    };
    pub const GREEN: Color = Color {
        red: 0,
        green: 128,
        blue: 0,
        alpha: 255,
        valid: true,
    };
    pub const GREEN_YELLOW: Color = Color {
        red: 173,
        green: 255,
        blue: 47,
        alpha: 255,
        valid: true,
    };
    pub const GREY: Color = Color {
        red: 128,
        green: 128,
        blue: 128,
        alpha: 255,
        valid: true,
    };
    pub const HONEYDEW: Color = Color {
        red: 240,
        green: 255,
        blue: 240,
        alpha: 255,
        valid: true,
    };
    pub const HOT_PINK: Color = Color {
        red: 255,
        green: 105,
        blue: 180,
        alpha: 255,
        valid: true,
    };
    pub const INDIAN_RED: Color = Color {
        red: 205,
        green: 92,
        blue: 92,
        alpha: 255,
        valid: true,
    };
    pub const INDIGO: Color = Color {
        red: 75,
        green: 0,
        blue: 130,
        alpha: 255,
        valid: true,
    };
    pub const IVORY: Color = Color {
        red: 255,
        green: 255,
        blue: 240,
        alpha: 255,
        valid: true,
    };
    pub const KHAKI: Color = Color {
        red: 240,
        green: 230,
        blue: 140,
        alpha: 255,
        valid: true,
    };
    pub const LAVENDER_BLUSH: Color = Color {
        red: 255,
        green: 240,
        blue: 245,
        alpha: 255,
        valid: true,
    };
    pub const LAVENDER: Color = Color {
        red: 230,
        green: 230,
        blue: 250,
        alpha: 255,
        valid: true,
    };
    pub const LAWN_GREEN: Color = Color {
        red: 124,
        green: 252,
        blue: 0,
        alpha: 255,
        valid: true,
    };
    pub const LEMON_CHIFFON: Color = Color {
        red: 255,
        green: 250,
        blue: 205,
        alpha: 255,
        valid: true,
    };
    pub const LIGHT_BLUE: Color = Color {
        red: 173,
        green: 216,
        blue: 230,
        alpha: 255,
        valid: true,
    };
    pub const LIGHT_CORAL: Color = Color {
        red: 240,
        green: 128,
        blue: 128,
        alpha: 255,
        valid: true,
    };
    pub const LIGHT_CYAN: Color = Color {
        red: 224,
        green: 255,
        blue: 255,
        alpha: 255,
        valid: true,
    };
    pub const LIGHT_GOLDENROD_YELLOW: Color = Color {
        red: 250,
        green: 250,
        blue: 210,
        alpha: 255,
        valid: true,
    };
    pub const LIGHT_GRAY: Color = Color {
        red: 211,
        green: 211,
        blue: 211,
        alpha: 255,
        valid: true,
    };
    pub const LIGHT_GREEN: Color = Color {
        red: 144,
        green: 238,
        blue: 144,
        alpha: 255,
        valid: true,
    };
    pub const LIGHT_GREY: Color = Color {
        red: 211,
        green: 211,
        blue: 211,
        alpha: 255,
        valid: true,
    };
    pub const LIGHT_PINK: Color = Color {
        red: 255,
        green: 182,
        blue: 193,
        alpha: 255,
        valid: true,
    };
    pub const LIGHT_SALMON: Color = Color {
        red: 255,
        green: 160,
        blue: 122,
        alpha: 255,
        valid: true,
    };
    pub const LIGHT_SEA_GREEN: Color = Color {
        red: 32,
        green: 178,
        blue: 170,
        alpha: 255,
        valid: true,
    };
    pub const LIGHT_SKY_BLUE: Color = Color {
        red: 135,
        green: 206,
        blue: 250,
        alpha: 255,
        valid: true,
    };
    pub const LIGHT_SLATE_GRAY: Color = Color {
        red: 119,
        green: 136,
        blue: 153,
        alpha: 255,
        valid: true,
    };
    pub const LIGHT_SLATE_GREY: Color = Color {
        red: 119,
        green: 136,
        blue: 153,
        alpha: 255,
        valid: true,
    };
    pub const LIGHT_STEEL_BLUE: Color = Color {
        red: 176,
        green: 196,
        blue: 222,
        alpha: 255,
        valid: true,
    };
    pub const LIGHT_YELLOW: Color = Color {
        red: 255,
        green: 255,
        blue: 224,
        alpha: 255,
        valid: true,
    };
    pub const LIME: Color = Color {
        red: 0,
        green: 255,
        blue: 0,
        alpha: 255,
        valid: true,
    };
    pub const LIME_GREEN: Color = Color {
        red: 50,
        green: 205,
        blue: 50,
        alpha: 255,
        valid: true,
    };
    pub const LINEN: Color = Color {
        red: 250,
        green: 240,
        blue: 230,
        alpha: 255,
        valid: true,
    };
    pub const MAGENTA: Color = Color {
        red: 255,
        green: 0,
        blue: 255,
        alpha: 255,
        valid: true,
    };
    pub const MAROON: Color = Color {
        red: 128,
        green: 0,
        blue: 0,
        alpha: 255,
        valid: true,
    };
    pub const MEDIUM_AQUAMARINE: Color = Color {
        red: 102,
        green: 205,
        blue: 170,
        alpha: 255,
        valid: true,
    };
    pub const MEDIUM_BLUE: Color = Color {
        red: 0,
        green: 0,
        blue: 205,
        alpha: 255,
        valid: true,
    };
    pub const MEDIUM_ORCHID: Color = Color {
        red: 186,
        green: 85,
        blue: 211,
        alpha: 255,
        valid: true,
    };
    pub const MEDIUM_PURPLE: Color = Color {
        red: 147,
        green: 112,
        blue: 219,
        alpha: 255,
        valid: true,
    };
    pub const MEDIUM_SEA_GREEN: Color = Color {
        red: 60,
        green: 179,
        blue: 113,
        alpha: 255,
        valid: true,
    };
    pub const MEDIUM_SLATE_BLUE: Color = Color {
        red: 123,
        green: 104,
        blue: 238,
        alpha: 255,
        valid: true,
    };
    pub const MEDIUM_SPRING_GREEN: Color = Color {
        red: 0,
        green: 250,
        blue: 154,
        alpha: 255,
        valid: true,
    };
    pub const MEDIUM_TURQUOISE: Color = Color {
        red: 72,
        green: 209,
        blue: 204,
        alpha: 255,
        valid: true,
    };
    pub const MEDIUM_VIOLET_RED: Color = Color {
        red: 199,
        green: 21,
        blue: 133,
        alpha: 255,
        valid: true,
    };
    pub const MIDNIGHT_BLUE: Color = Color {
        red: 25,
        green: 25,
        blue: 112,
        alpha: 255,
        valid: true,
    };
    pub const MINT_CREAM: Color = Color {
        red: 245,
        green: 255,
        blue: 250,
        alpha: 255,
        valid: true,
    };
    pub const MISTY_ROSE: Color = Color {
        red: 255,
        green: 228,
        blue: 225,
        alpha: 255,
        valid: true,
    };
    pub const MOCCASIN: Color = Color {
        red: 255,
        green: 228,
        blue: 181,
        alpha: 255,
        valid: true,
    };
    pub const NAVAJO_WHITE: Color = Color {
        red: 255,
        green: 222,
        blue: 173,
        alpha: 255,
        valid: true,
    };
    pub const NAVY: Color = Color {
        red: 0,
        green: 0,
        blue: 128,
        alpha: 255,
        valid: true,
    };
    pub const OLD_LACE: Color = Color {
        red: 253,
        green: 245,
        blue: 230,
        alpha: 255,
        valid: true,
    };
    pub const OLIVE: Color = Color {
        red: 128,
        green: 128,
        blue: 0,
        alpha: 255,
        valid: true,
    };
    pub const OLIVE_DRAB: Color = Color {
        red: 107,
        green: 142,
        blue: 35,
        alpha: 255,
        valid: true,
    };
    pub const ORANGE: Color = Color {
        red: 255,
        green: 165,
        blue: 0,
        alpha: 255,
        valid: true,
    };
    pub const ORANGE_RED: Color = Color {
        red: 255,
        green: 69,
        blue: 0,
        alpha: 255,
        valid: true,
    };
    pub const ORCHID: Color = Color {
        red: 218,
        green: 112,
        blue: 214,
        alpha: 255,
        valid: true,
    };
    pub const PALE_GOLDENROD: Color = Color {
        red: 238,
        green: 232,
        blue: 170,
        alpha: 255,
        valid: true,
    };
    pub const PALE_GREEN: Color = Color {
        red: 152,
        green: 251,
        blue: 152,
        alpha: 255,
        valid: true,
    };
    pub const PALE_TURQUOISE: Color = Color {
        red: 175,
        green: 238,
        blue: 238,
        alpha: 255,
        valid: true,
    };
    pub const PALE_VIOLET_RED: Color = Color {
        red: 219,
        green: 112,
        blue: 147,
        alpha: 255,
        valid: true,
    };
    pub const PAPAYA_WHIP: Color = Color {
        red: 255,
        green: 239,
        blue: 213,
        alpha: 255,
        valid: true,
    };
    pub const PEACH_PUFF: Color = Color {
        red: 255,
        green: 218,
        blue: 185,
        alpha: 255,
        valid: true,
    };
    pub const PERU: Color = Color {
        red: 205,
        green: 133,
        blue: 63,
        alpha: 255,
        valid: true,
    };
    pub const PINK: Color = Color {
        red: 255,
        green: 192,
        blue: 203,
        alpha: 255,
        valid: true,
    };
    pub const PLUM: Color = Color {
        red: 221,
        green: 160,
        blue: 221,
        alpha: 255,
        valid: true,
    };
    pub const POWDER_BLUE: Color = Color {
        red: 176,
        green: 224,
        blue: 230,
        alpha: 255,
        valid: true,
    };
    pub const PURPLE: Color = Color {
        red: 128,
        green: 0,
        blue: 128,
        alpha: 255,
        valid: true,
    };
    pub const REBECCA_PURPLE: Color = Color {
        red: 102,
        green: 51,
        blue: 153,
        alpha: 255,
        valid: true,
    };
    pub const RED: Color = Color {
        red: 255,
        green: 0,
        blue: 0,
        alpha: 255,
        valid: true,
    };
    pub const ROSY_BROWN: Color = Color {
        red: 188,
        green: 143,
        blue: 143,
        alpha: 255,
        valid: true,
    };
    pub const ROYAL_BLUE: Color = Color {
        red: 65,
        green: 105,
        blue: 225,
        alpha: 255,
        valid: true,
    };
    pub const SADDLE_BROWN: Color = Color {
        red: 139,
        green: 69,
        blue: 19,
        alpha: 255,
        valid: true,
    };
    pub const SALMON: Color = Color {
        red: 250,
        green: 128,
        blue: 114,
        alpha: 255,
        valid: true,
    };
    pub const SANDY_BROWN: Color = Color {
        red: 244,
        green: 164,
        blue: 96,
        alpha: 255,
        valid: true,
    };
    pub const SEA_GREEN: Color = Color {
        red: 46,
        green: 139,
        blue: 87,
        alpha: 255,
        valid: true,
    };
    pub const SEASHELL: Color = Color {
        red: 255,
        green: 245,
        blue: 238,
        alpha: 255,
        valid: true,
    };
    pub const SIENNA: Color = Color {
        red: 160,
        green: 82,
        blue: 45,
        alpha: 255,
        valid: true,
    };
    pub const SILVER: Color = Color {
        red: 192,
        green: 192,
        blue: 192,
        alpha: 255,
        valid: true,
    };
    pub const SKY_BLUE: Color = Color {
        red: 135,
        green: 206,
        blue: 235,
        alpha: 255,
        valid: true,
    };
    pub const SLATE_BLUE: Color = Color {
        red: 106,
        green: 90,
        blue: 205,
        alpha: 255,
        valid: true,
    };
    pub const SLATE_GRAY: Color = Color {
        red: 112,
        green: 128,
        blue: 144,
        alpha: 255,
        valid: true,
    };
    pub const SLATE_GREY: Color = Color {
        red: 112,
        green: 128,
        blue: 144,
        alpha: 255,
        valid: true,
    };
    pub const SNOW: Color = Color {
        red: 255,
        green: 250,
        blue: 250,
        alpha: 255,
        valid: true,
    };
    pub const SPRING_GREEN: Color = Color {
        red: 0,
        green: 255,
        blue: 127,
        alpha: 255,
        valid: true,
    };
    pub const STEEL_BLUE: Color = Color {
        red: 70,
        green: 130,
        blue: 180,
        alpha: 255,
        valid: true,
    };
    pub const TAN: Color = Color {
        red: 210,
        green: 180,
        blue: 140,
        alpha: 255,
        valid: true,
    };
    pub const TEAL: Color = Color {
        red: 0,
        green: 128,
        blue: 128,
        alpha: 255,
        valid: true,
    };
    pub const THISTLE: Color = Color {
        red: 216,
        green: 191,
        blue: 216,
        alpha: 255,
        valid: true,
    };
    pub const TOMATO: Color = Color {
        red: 255,
        green: 99,
        blue: 71,
        alpha: 255,
        valid: true,
    };
    pub const TURQUOISE: Color = Color {
        red: 64,
        green: 224,
        blue: 208,
        alpha: 255,
        valid: true,
    };
    pub const VIOLET: Color = Color {
        red: 238,
        green: 130,
        blue: 238,
        alpha: 255,
        valid: true,
    };
    pub const WHEAT: Color = Color {
        red: 245,
        green: 222,
        blue: 179,
        alpha: 255,
        valid: true,
    };
    pub const WHITE: Color = Color {
        red: 255,
        green: 255,
        blue: 255,
        alpha: 255,
        valid: true,
    };
    pub const WHITE_SMOKE: Color = Color {
        red: 245,
        green: 245,
        blue: 245,
        alpha: 255,
        valid: true,
    };
    pub const YELLOW_GREEN: Color = Color {
        red: 255,
        green: 255,
        blue: 0,
        alpha: 255,
        valid: true,
    };
    pub const YELLOW: Color = Color {
        red: 154,
        green: 205,
        blue: 50,
        alpha: 255,
        valid: true,
    };
}
