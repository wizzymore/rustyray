pub type Color = rustyray_sys::color::Color;

impl ColorPallete for Color {}

pub trait ColorPallete {
    const INDIANRED: Color = Color::new(205, 92, 92, 255);
    const LIGHTCORAL: Color = Color::new(240, 128, 128, 255);
    const SALMON: Color = Color::new(250, 128, 114, 255);
    const DARKSALMON: Color = Color::new(233, 150, 122, 255);
    const LIGHTSALMON: Color = Color::new(255, 160, 122, 255);
    const CRIMSON: Color = Color::new(220, 20, 60, 255);
    const FIREBRICK: Color = Color::new(178, 34, 34, 255);
    const DARKRED: Color = Color::new(139, 0, 0, 255);
    const LIGHTPINK: Color = Color::new(255, 182, 193, 255);
    const HOTPINK: Color = Color::new(255, 105, 180, 255);
    const DEEPPINK: Color = Color::new(255, 20, 147, 255);
    const MEDIUMVIOLETRED: Color = Color::new(199, 21, 133, 255);
    const PALEVIOLETRED: Color = Color::new(219, 112, 147, 255);
    const CORAL: Color = Color::new(255, 127, 80, 255);
    const TOMATO: Color = Color::new(255, 99, 71, 255);
    const ORANGERED: Color = Color::new(255, 69, 0, 255);
    const DARKORANGE: Color = Color::new(255, 140, 0, 255);
    const LIGHTYELLOW: Color = Color::new(255, 255, 224, 255);
    const LEMONCHIFFON: Color = Color::new(255, 250, 205, 255);
    const LIGHTGOLDENRODYELLOW: Color = Color::new(250, 250, 210, 255);
    const PAPAYAWHIP: Color = Color::new(255, 239, 213, 255);
    const MOCCASIN: Color = Color::new(255, 228, 181, 255);
    const PEACHPUFF: Color = Color::new(255, 218, 185, 255);
    const PALEGOLDENROD: Color = Color::new(238, 232, 170, 255);
    const KHAKI: Color = Color::new(240, 230, 140, 255);
    const DARKKHAKI: Color = Color::new(189, 183, 107, 255);
    const LAVENDER: Color = Color::new(230, 230, 250, 255);
    const THISTLE: Color = Color::new(216, 191, 216, 255);
    const PLUM: Color = Color::new(221, 160, 221, 255);
    const ORCHID: Color = Color::new(218, 112, 214, 255);
    const FUCHSIA: Color = Color::new(255, 0, 255, 255);
    const MAGENTA: Color = Color::new(255, 0, 255, 255);
    const MEDIUMORCHID: Color = Color::new(186, 85, 211, 255);
    const MEDIUMPURPLE: Color = Color::new(147, 112, 219, 255);
    const REBECCAPURPLE: Color = Color::new(102, 51, 153, 255);
    const BLUEVIOLET: Color = Color::new(138, 43, 226, 255);
    const DARKVIOLET: Color = Color::new(148, 0, 211, 255);
    const DARKORCHID: Color = Color::new(153, 50, 204, 255);
    const DARKMAGENTA: Color = Color::new(139, 0, 139, 255);
    const DARKPURPLE: Color = Color::new(112, 31, 126, 255);
    const INDIGO: Color = Color::new(75, 0, 130, 255);
    const SLATEBLUE: Color = Color::new(106, 90, 205, 255);
    const DARKSLATEBLUE: Color = Color::new(72, 61, 139, 255);
    const MEDIUMSLATEBLUE: Color = Color::new(123, 104, 238, 255);
    const GREENYELLOW: Color = Color::new(173, 255, 47, 255);
    const CHARTREUSE: Color = Color::new(127, 255, 0, 255);
    const LAWNGREEN: Color = Color::new(124, 252, 0, 255);
    const LIMEGREEN: Color = Color::new(50, 205, 50, 255);
    const PALEGREEN: Color = Color::new(152, 251, 152, 255);
    const LIGHTGREEN: Color = Color::new(144, 238, 144, 255);
    const MEDIUMSPRINGGREEN: Color = Color::new(0, 250, 154, 255);
    const SPRINGGREEN: Color = Color::new(0, 255, 127, 255);
    const MEDIUMSEAGREEN: Color = Color::new(60, 179, 113, 255);
    const SEAGREEN: Color = Color::new(46, 139, 87, 255);
    const FORESTGREEN: Color = Color::new(34, 139, 34, 255);
    const DARKGREEN: Color = Color::new(0, 117, 44, 255);
    const YELLOWGREEN: Color = Color::new(154, 205, 50, 255);
    const OLIVEDRAB: Color = Color::new(107, 142, 35, 255);
    const OLIVE: Color = Color::new(128, 128, 0, 255);
    const DARKOLIVEGREEN: Color = Color::new(85, 107, 47, 255);
    const MEDIUMAQUAMARINE: Color = Color::new(102, 205, 170, 255);
    const DARKSEAGREEN: Color = Color::new(143, 188, 139, 255);
    const LIGHTSEAGREEN: Color = Color::new(32, 178, 170, 255);
    const DARKCYAN: Color = Color::new(0, 139, 139, 255);
    const TEAL: Color = Color::new(0, 128, 128, 255);
    const AQUA: Color = Color::new(0, 255, 255, 255);
    const CYAN: Color = Color::new(0, 255, 255, 255);
    const LIGHTCYAN: Color = Color::new(224, 255, 255, 255);
    const PALETURQUOISE: Color = Color::new(175, 238, 238, 255);
    const AQUAMARINE: Color = Color::new(127, 255, 212, 255);
    const TURQUOISE: Color = Color::new(64, 224, 208, 255);
    const MEDIUMTURQUOISE: Color = Color::new(72, 209, 204, 255);
    const DARKTURQUOISE: Color = Color::new(0, 206, 209, 255);
    const CADETBLUE: Color = Color::new(95, 158, 160, 255);
    const STEELBLUE: Color = Color::new(70, 130, 180, 255);
    const LIGHTSTEELBLUE: Color = Color::new(176, 196, 222, 255);
    const POWDERBLUE: Color = Color::new(176, 224, 230, 255);
    const LIGHTBLUE: Color = Color::new(173, 216, 230, 255);
    const LIGHTSKYBLUE: Color = Color::new(135, 206, 250, 255);
    const DEEPSKYBLUE: Color = Color::new(0, 191, 255, 255);
    const DODGERBLUE: Color = Color::new(30, 144, 255, 255);
    const CORNFLOWERBLUE: Color = Color::new(100, 149, 237, 255);
    const ROYALBLUE: Color = Color::new(65, 105, 225, 255);
    const MEDIUMBLUE: Color = Color::new(0, 0, 205, 255);
    const DARKBLUE: Color = Color::new(0, 82, 172, 255);
    const NAVY: Color = Color::new(0, 0, 128, 255);
    const MIDNIGHTBLUE: Color = Color::new(25, 25, 112, 255);
    const CORNSILK: Color = Color::new(255, 248, 220, 255);
    const BLANCHEDALMOND: Color = Color::new(255, 235, 205, 255);
    const BISQUE: Color = Color::new(255, 228, 196, 255);
    const NAVAJOWHITE: Color = Color::new(255, 222, 173, 255);
    const WHEAT: Color = Color::new(245, 222, 179, 255);
    const BURLYWOOD: Color = Color::new(222, 184, 135, 255);
    const TAN: Color = Color::new(210, 180, 140, 255);
    const ROSYBROWN: Color = Color::new(188, 143, 143, 255);
    const SANDYBROWN: Color = Color::new(244, 164, 96, 255);
    const GOLDENROD: Color = Color::new(218, 165, 32, 255);
    const DARKGOLDENROD: Color = Color::new(184, 134, 11, 255);
    const PERU: Color = Color::new(205, 133, 63, 255);
    const CHOCOLATE: Color = Color::new(210, 105, 30, 255);
    const SADDLEBROWN: Color = Color::new(139, 69, 19, 255);
    const SIENNA: Color = Color::new(160, 82, 45, 255);
    const DARKBROWN: Color = Color::new(76, 63, 47, 255);
    const MAROON: Color = Color::new(190, 33, 55, 255);
    const WHITE: Color = Color::new(255, 255, 255, 255);
    const SNOW: Color = Color::new(255, 250, 250, 255);
    const HONEYDEW: Color = Color::new(240, 255, 240, 255);
    const MINTCREAM: Color = Color::new(245, 255, 250, 255);
    const AZURE: Color = Color::new(240, 255, 255, 255);
    const ALICEBLUE: Color = Color::new(240, 248, 255, 255);
    const GHOSTWHITE: Color = Color::new(248, 248, 255, 255);
    const WHITESMOKE: Color = Color::new(245, 245, 245, 255);
    const SEASHELL: Color = Color::new(255, 245, 238, 255);
    const OLDLACE: Color = Color::new(253, 245, 230, 255);
    const FLORALWHITE: Color = Color::new(255, 250, 240, 255);
    const IVORY: Color = Color::new(255, 255, 240, 255);
    const ANTIQUEWHITE: Color = Color::new(250, 235, 215, 255);
    const LINEN: Color = Color::new(250, 240, 230, 255);
    const LAVENDERBLUSH: Color = Color::new(255, 240, 245, 255);
    const MISTYROSE: Color = Color::new(255, 228, 225, 255);
    const GAINSBORO: Color = Color::new(220, 220, 220, 255);
    const SILVER: Color = Color::new(192, 192, 192, 255);
    const DARKGRAY: Color = Color::new(80, 80, 80, 255);
    const GRAY: Color = Color::new(130, 130, 130, 255);
    const DIMGRAY: Color = Color::new(105, 105, 105, 255);
    const LIGHTSLATEGRAY: Color = Color::new(119, 136, 153, 255);
    const SLATEGRAY: Color = Color::new(112, 128, 144, 255);
    const DARKSLATEGRAY: Color = Color::new(47, 79, 79, 255);
}
