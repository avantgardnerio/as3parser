use std::collections::HashMap;
use lazy_static::lazy_static;
use maplit::hashmap;

/// Converts a CSS [color constant](http://www.w3schools.com/css/css_colorsfull.asp) into an integer.
pub fn css_color_constant_to_int(name: &str) -> Option<u32> {
    COLOR_MAP.get(&name.to_lowercase()).map(|i| *i)
}

lazy_static! {
    /// Map color constant names to 24-bit RGB integer values.
    ///
    /// See also: [CSS colors](http://www.w3schools.com/css/css_colorsfull.asp)
    static ref COLOR_MAP: HashMap<String, u32> = hashmap! {
        "black".into() => 0x000000,
        "navy".into() => 0x000080,
        "darkblue".into() => 0x00008b,
        "mediumblue".into() => 0x0000cd,
        "blue".into() => 0x0000ff,
        "darkgreen".into() => 0x006400,
        "green".into() => 0x008000,
        "teal".into() => 0x008080,
        "darkcyan".into() => 0x008b8b,
        "deepskyblue".into() => 0x00bfff,
        "darkturquoise".into() => 0x00ced1,
        "mediumspringgreen".into() => 0x00fa9a,
        "lime".into() => 0x00ff00,
        "springgreen".into() => 0x00ff7f,
        "aqua".into() => 0x00ffff,
        "cyan".into() => 0x00ffff,
        "midnightblue".into() => 0x191970,
        "dodgerblue".into() => 0x1e90ff,
        "lightseagreen".into() => 0x20b2aa,
        "forestgreen".into() => 0x228b22,
        "seagreen".into() => 0x2e8b57,
        "darkslategray".into() => 0x2f4f4f,
        "darkslategrey".into() => 0x2f4f4f,
        "limegreen".into() => 0x32cd32,
        "mediumseagreen".into() => 0x3cb371,
        "turquoise".into() => 0x40e0d0,
        "royalblue".into() => 0x4169e1,
        "steelblue".into() => 0x4682b4,
        "darkslateblue".into() => 0x483d8b,
        "mediumturquoise".into() => 0x48d1cc,
        "indigo ".into() => 0x4b0082,
        "darkolivegreen".into() => 0x556b2f,
        "cadetblue".into() => 0x5f9ea0,
        "cornflowerblue".into() => 0x6495ed,
        "mediumaquamarine".into() => 0x66cdaa,
        "dimgray".into() => 0x696969,
        "dimgrey".into() => 0x696969,
        "slateblue".into() => 0x6a5acd,
        "olivedrab".into() => 0x6b8e23,
        "slategray".into() => 0x708090,
        "slategrey".into() => 0x708090,
        "lightslategray".into() => 0x778899,
        "lightslategrey".into() => 0x778899,
        "mediumslateblue".into() => 0x7b68ee,
        "lawngreen".into() => 0x7cfc00,
        "chartreuse".into() => 0x7fff00,
        "aquamarine".into() => 0x7fffd4,
        "maroon".into() => 0x800000,
        "purple".into() => 0x800080,
        "olive".into() => 0x808000,
        "gray".into() => 0x808080,
        "grey".into() => 0x808080,
        "skyblue".into() => 0x87ceeb,
        "lightskyblue".into() => 0x87cefa,
        "blueviolet".into() => 0x8a2be2,
        "darkred".into() => 0x8b0000,
        "darkmagenta".into() => 0x8b008b,
        "saddlebrown".into() => 0x8b4513,
        "darkseagreen".into() => 0x8fbc8f,
        "lightgreen".into() => 0x90ee90,
        "mediumpurple".into() => 0x9370d8,
        "darkviolet".into() => 0x9400d3,
        "palegreen".into() => 0x98fb98,
        "darkorchid".into() => 0x9932cc,
        "yellowgreen".into() => 0x9acd32,
        "sienna".into() => 0xa0522d,
        "brown".into() => 0xa52a2a,
        "darkgray".into() => 0xa9a9a9,
        "darkgrey".into() => 0xa9a9a9,
        "lightblue".into() => 0xadd8e6,
        "greenyellow".into() => 0xadff2f,
        "paleturquoise".into() => 0xafeeee,
        "lightsteelblue".into() => 0xb0c4de,
        "powderblue".into() => 0xb0e0e6,
        "firebrick".into() => 0xb22222,
        "darkgoldenrod".into() => 0xb8860b,
        "mediumorchid".into() => 0xba55d3,
        "rosybrown".into() => 0xbc8f8f,
        "darkkhaki".into() => 0xbdb76b,
        "silver".into() => 0xc0c0c0,
        "mediumvioletred".into() => 0xc71585,
        "indianred ".into() => 0xcd5c5c,
        "peru".into() => 0xcd853f,
        "chocolate".into() => 0xd2691e,
        "tan".into() => 0xd2b48c,
        "lightgray".into() => 0xd3d3d3,
        "lightgrey".into() => 0xd3d3d3,
        "palevioletred".into() => 0xd87093,
        "thistle".into() => 0xd8bfd8,
        "orchid".into() => 0xda70d6,
        "goldenrod".into() => 0xdaa520,
        "crimson".into() => 0xdc143c,
        "gainsboro".into() => 0xdcdcdc,
        "plum".into() => 0xdda0dd,
        "burlywood".into() => 0xdeb887,
        "lightcyan".into() => 0xe0ffff,
        "lavender".into() => 0xe6e6fa,
        "darksalmon".into() => 0xe9967a,
        "violet".into() => 0xee82ee,
        "palegoldenrod".into() => 0xeee8aa,
        "lightcoral".into() => 0xf08080,
        "khaki".into() => 0xf0e68c,
        "aliceblue".into() => 0xf0f8ff,
        "honeydew".into() => 0xf0fff0,
        "azure".into() => 0xf0ffff,
        "sandybrown".into() => 0xf4a460,
        "wheat".into() => 0xf5deb3,
        "beige".into() => 0xf5f5dc,
        "whitesmoke".into() => 0xf5f5f5,
        "mintcream".into() => 0xf5fffa,
        "ghostwhite".into() => 0xf8f8ff,
        "salmon".into() => 0xfa8072,
        "antiquewhite".into() => 0xfaebd7,
        "linen".into() => 0xfaf0e6,
        "lightgoldenrodyellow".into() => 0xfafad2,
        "oldlace".into() => 0xfdf5e6,
        "red".into() => 0xff0000,
        "fuchsia".into() => 0xff00ff,
        "magenta".into() => 0xff00ff,
        "deeppink".into() => 0xff1493,
        "orangered".into() => 0xff4500,
        "tomato".into() => 0xff6347,
        "hotpink".into() => 0xff69b4,
        "coral".into() => 0xff7f50,
        "darkorange".into() => 0xff8c00,
        "lightsalmon".into() => 0xffa07a,
        "orange".into() => 0xffa500,
        "lightpink".into() => 0xffb6c1,
        "pink".into() => 0xffc0cb,
        "gold".into() => 0xffd700,
        "peachpuff".into() => 0xffdab9,
        "navajowhite".into() => 0xffdead,
        "moccasin".into() => 0xffe4b5,
        "bisque".into() => 0xffe4c4,
        "mistyrose".into() => 0xffe4e1,
        "blanchedalmond".into() => 0xffebcd,
        "papayawhip".into() => 0xffefd5,
        "lavenderblush".into() => 0xfff0f5,
        "seashell".into() => 0xfff5ee,
        "cornsilk".into() => 0xfff8dc,
        "lemonchiffon".into() => 0xfffacd,
        "floralwhite".into() => 0xfffaf0,
        "snow".into() => 0xfffafa,
        "yellow".into() => 0xffff00,
        "lightyellow".into() => 0xffffe0,
        "ivory".into() => 0xfffff0,
        "white".into() => 0xffffff,
    };
}