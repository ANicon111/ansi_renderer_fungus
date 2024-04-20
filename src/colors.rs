use crate::color::Color;

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
