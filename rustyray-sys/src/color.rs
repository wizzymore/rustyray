#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Default for Color {
    fn default() -> Self {
        Color::WHITE
    }
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Get color with alpha applied, alpha goes from 0.0 to 1.0
    pub fn fade(mut self, alpha: f32) -> Self {
        self.a = (255.0 * alpha.clamp(0., 1.)) as u8;
        self
    }
}

impl Color {
    /// Get hexadecimal value for a [Color] (0xRRGGBBAA)
    pub fn to_int(&self) -> i32 {
        ((self.r as i32) << 24) | ((self.g as i32) << 16) | ((self.b as i32) << 8) | self.a as i32
    }

    /// Get [Color] multiplied with another [Color]
    pub fn tint(&mut self, color: &Color) -> &mut Self {
        self.r = ((self.r as i32 * color.r as i32) / 255) as u8;
        self.g = ((self.g as i32 * color.g as i32) / 255) as u8;
        self.b = ((self.b as i32 * color.b as i32) / 255) as u8;
        self.a = ((self.a as i32 * color.a as i32) / 255) as u8;

        self
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8, u8)) -> Self {
        Color::new(value.0, value.1, value.2, value.3)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        Color::new(value.0, value.1, value.2, 255)
    }
}

impl Color {
    pub const INDIANRED: Color = Color::new(205, 92, 92, 255);
    pub const LIGHTCORAL: Color = Color::new(240, 128, 128, 255);
    pub const SALMON: Color = Color::new(250, 128, 114, 255);
    pub const DARKSALMON: Color = Color::new(233, 150, 122, 255);
    pub const LIGHTSALMON: Color = Color::new(255, 160, 122, 255);
    pub const CRIMSON: Color = Color::new(220, 20, 60, 255);
    pub const RED: Color = Color::new(230, 41, 55, 255);
    pub const FIREBRICK: Color = Color::new(178, 34, 34, 255);
    pub const DARKRED: Color = Color::new(139, 0, 0, 255);
    pub const PINK: Color = Color::new(255, 109, 194, 255);
    pub const LIGHTPINK: Color = Color::new(255, 182, 193, 255);
    pub const HOTPINK: Color = Color::new(255, 105, 180, 255);
    pub const DEEPPINK: Color = Color::new(255, 20, 147, 255);
    pub const MEDIUMVIOLETRED: Color = Color::new(199, 21, 133, 255);
    pub const PALEVIOLETRED: Color = Color::new(219, 112, 147, 255);
    pub const CORAL: Color = Color::new(255, 127, 80, 255);
    pub const TOMATO: Color = Color::new(255, 99, 71, 255);
    pub const ORANGERED: Color = Color::new(255, 69, 0, 255);
    pub const DARKORANGE: Color = Color::new(255, 140, 0, 255);
    pub const ORANGE: Color = Color::new(255, 161, 0, 255);
    pub const GOLD: Color = Color::new(255, 203, 0, 255);
    pub const YELLOW: Color = Color::new(253, 249, 0, 255);
    pub const LIGHTYELLOW: Color = Color::new(255, 255, 224, 255);
    pub const LEMONCHIFFON: Color = Color::new(255, 250, 205, 255);
    pub const LIGHTGOLDENRODYELLOW: Color = Color::new(250, 250, 210, 255);
    pub const PAPAYAWHIP: Color = Color::new(255, 239, 213, 255);
    pub const MOCCASIN: Color = Color::new(255, 228, 181, 255);
    pub const PEACHPUFF: Color = Color::new(255, 218, 185, 255);
    pub const PALEGOLDENROD: Color = Color::new(238, 232, 170, 255);
    pub const KHAKI: Color = Color::new(240, 230, 140, 255);
    pub const DARKKHAKI: Color = Color::new(189, 183, 107, 255);
    pub const LAVENDER: Color = Color::new(230, 230, 250, 255);
    pub const THISTLE: Color = Color::new(216, 191, 216, 255);
    pub const PLUM: Color = Color::new(221, 160, 221, 255);
    pub const VIOLET: Color = Color::new(135, 60, 190, 255);
    pub const ORCHID: Color = Color::new(218, 112, 214, 255);
    pub const FUCHSIA: Color = Color::new(255, 0, 255, 255);
    pub const MAGENTA: Color = Color::new(255, 0, 255, 255);
    pub const MEDIUMORCHID: Color = Color::new(186, 85, 211, 255);
    pub const MEDIUMPURPLE: Color = Color::new(147, 112, 219, 255);
    pub const REBECCAPURPLE: Color = Color::new(102, 51, 153, 255);
    pub const BLUEVIOLET: Color = Color::new(138, 43, 226, 255);
    pub const DARKVIOLET: Color = Color::new(148, 0, 211, 255);
    pub const DARKORCHID: Color = Color::new(153, 50, 204, 255);
    pub const DARKMAGENTA: Color = Color::new(139, 0, 139, 255);
    pub const PURPLE: Color = Color::new(200, 122, 255, 255);
    pub const DARKPURPLE: Color = Color::new(112, 31, 126, 255);
    pub const INDIGO: Color = Color::new(75, 0, 130, 255);
    pub const SLATEBLUE: Color = Color::new(106, 90, 205, 255);
    pub const DARKSLATEBLUE: Color = Color::new(72, 61, 139, 255);
    pub const MEDIUMSLATEBLUE: Color = Color::new(123, 104, 238, 255);
    pub const GREENYELLOW: Color = Color::new(173, 255, 47, 255);
    pub const CHARTREUSE: Color = Color::new(127, 255, 0, 255);
    pub const LAWNGREEN: Color = Color::new(124, 252, 0, 255);
    pub const LIME: Color = Color::new(0, 158, 47, 255);
    pub const LIMEGREEN: Color = Color::new(50, 205, 50, 255);
    pub const PALEGREEN: Color = Color::new(152, 251, 152, 255);
    pub const LIGHTGREEN: Color = Color::new(144, 238, 144, 255);
    pub const MEDIUMSPRINGGREEN: Color = Color::new(0, 250, 154, 255);
    pub const SPRINGGREEN: Color = Color::new(0, 255, 127, 255);
    pub const MEDIUMSEAGREEN: Color = Color::new(60, 179, 113, 255);
    pub const SEAGREEN: Color = Color::new(46, 139, 87, 255);
    pub const FORESTGREEN: Color = Color::new(34, 139, 34, 255);
    pub const GREEN: Color = Color::new(0, 228, 48, 255);
    pub const DARKGREEN: Color = Color::new(0, 117, 44, 255);
    pub const YELLOWGREEN: Color = Color::new(154, 205, 50, 255);
    pub const OLIVEDRAB: Color = Color::new(107, 142, 35, 255);
    pub const OLIVE: Color = Color::new(128, 128, 0, 255);
    pub const DARKOLIVEGREEN: Color = Color::new(85, 107, 47, 255);
    pub const MEDIUMAQUAMARINE: Color = Color::new(102, 205, 170, 255);
    pub const DARKSEAGREEN: Color = Color::new(143, 188, 139, 255);
    pub const LIGHTSEAGREEN: Color = Color::new(32, 178, 170, 255);
    pub const DARKCYAN: Color = Color::new(0, 139, 139, 255);
    pub const TEAL: Color = Color::new(0, 128, 128, 255);
    pub const AQUA: Color = Color::new(0, 255, 255, 255);
    pub const CYAN: Color = Color::new(0, 255, 255, 255);
    pub const LIGHTCYAN: Color = Color::new(224, 255, 255, 255);
    pub const PALETURQUOISE: Color = Color::new(175, 238, 238, 255);
    pub const AQUAMARINE: Color = Color::new(127, 255, 212, 255);
    pub const TURQUOISE: Color = Color::new(64, 224, 208, 255);
    pub const MEDIUMTURQUOISE: Color = Color::new(72, 209, 204, 255);
    pub const DARKTURQUOISE: Color = Color::new(0, 206, 209, 255);
    pub const CADETBLUE: Color = Color::new(95, 158, 160, 255);
    pub const STEELBLUE: Color = Color::new(70, 130, 180, 255);
    pub const LIGHTSTEELBLUE: Color = Color::new(176, 196, 222, 255);
    pub const POWDERBLUE: Color = Color::new(176, 224, 230, 255);
    pub const LIGHTBLUE: Color = Color::new(173, 216, 230, 255);
    pub const SKYBLUE: Color = Color::new(102, 191, 255, 255);
    pub const LIGHTSKYBLUE: Color = Color::new(135, 206, 250, 255);
    pub const DEEPSKYBLUE: Color = Color::new(0, 191, 255, 255);
    pub const DODGERBLUE: Color = Color::new(30, 144, 255, 255);
    pub const CORNFLOWERBLUE: Color = Color::new(100, 149, 237, 255);
    pub const ROYALBLUE: Color = Color::new(65, 105, 225, 255);
    pub const BLUE: Color = Color::new(0, 121, 241, 255);
    pub const MEDIUMBLUE: Color = Color::new(0, 0, 205, 255);
    pub const DARKBLUE: Color = Color::new(0, 82, 172, 255);
    pub const NAVY: Color = Color::new(0, 0, 128, 255);
    pub const MIDNIGHTBLUE: Color = Color::new(25, 25, 112, 255);
    pub const CORNSILK: Color = Color::new(255, 248, 220, 255);
    pub const BLANCHEDALMOND: Color = Color::new(255, 235, 205, 255);
    pub const BISQUE: Color = Color::new(255, 228, 196, 255);
    pub const NAVAJOWHITE: Color = Color::new(255, 222, 173, 255);
    pub const WHEAT: Color = Color::new(245, 222, 179, 255);
    pub const BURLYWOOD: Color = Color::new(222, 184, 135, 255);
    pub const TAN: Color = Color::new(210, 180, 140, 255);
    pub const ROSYBROWN: Color = Color::new(188, 143, 143, 255);
    pub const SANDYBROWN: Color = Color::new(244, 164, 96, 255);
    pub const GOLDENROD: Color = Color::new(218, 165, 32, 255);
    pub const DARKGOLDENROD: Color = Color::new(184, 134, 11, 255);
    pub const PERU: Color = Color::new(205, 133, 63, 255);
    pub const CHOCOLATE: Color = Color::new(210, 105, 30, 255);
    pub const SADDLEBROWN: Color = Color::new(139, 69, 19, 255);
    pub const SIENNA: Color = Color::new(160, 82, 45, 255);
    pub const BROWN: Color = Color::new(127, 106, 79, 255);
    pub const DARKBROWN: Color = Color::new(76, 63, 47, 255);
    pub const MAROON: Color = Color::new(190, 33, 55, 255);
    pub const WHITE: Color = Color::new(255, 255, 255, 255);
    pub const SNOW: Color = Color::new(255, 250, 250, 255);
    pub const HONEYDEW: Color = Color::new(240, 255, 240, 255);
    pub const MINTCREAM: Color = Color::new(245, 255, 250, 255);
    pub const AZURE: Color = Color::new(240, 255, 255, 255);
    pub const ALICEBLUE: Color = Color::new(240, 248, 255, 255);
    pub const GHOSTWHITE: Color = Color::new(248, 248, 255, 255);
    pub const WHITESMOKE: Color = Color::new(245, 245, 245, 255);
    pub const SEASHELL: Color = Color::new(255, 245, 238, 255);
    pub const BEIGE: Color = Color::new(211, 176, 131, 255);
    pub const OLDLACE: Color = Color::new(253, 245, 230, 255);
    pub const FLORALWHITE: Color = Color::new(255, 250, 240, 255);
    pub const IVORY: Color = Color::new(255, 255, 240, 255);
    pub const ANTIQUEWHITE: Color = Color::new(250, 235, 215, 255);
    pub const LINEN: Color = Color::new(250, 240, 230, 255);
    pub const LAVENDERBLUSH: Color = Color::new(255, 240, 245, 255);
    pub const MISTYROSE: Color = Color::new(255, 228, 225, 255);
    pub const GAINSBORO: Color = Color::new(220, 220, 220, 255);
    pub const LIGHTGRAY: Color = Color::new(200, 200, 200, 255);
    pub const SILVER: Color = Color::new(192, 192, 192, 255);
    pub const DARKGRAY: Color = Color::new(80, 80, 80, 255);
    pub const GRAY: Color = Color::new(130, 130, 130, 255);
    pub const DIMGRAY: Color = Color::new(105, 105, 105, 255);
    pub const LIGHTSLATEGRAY: Color = Color::new(119, 136, 153, 255);
    pub const SLATEGRAY: Color = Color::new(112, 128, 144, 255);
    pub const DARKSLATEGRAY: Color = Color::new(47, 79, 79, 255);
    pub const BLACK: Color = Color::new(0, 0, 0, 255);
    pub const BLANK: Color = Color::new(0, 0, 0, 0);
    pub const RAYWHITE: Color = Color::new(245, 245, 245, 255);
}
