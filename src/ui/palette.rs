use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;


pub struct Palette {
    pub bg: [Rgb<Srgb, u8>; 8],
    pub fg: [Rgb<Srgb, u8>; 8]
}

impl Palette {
    pub fn dark() -> Self {
        let bg = [
            Rgb::new( 16,   0,  43),
            Rgb::new( 36,   0,  70),
            Rgb::new( 60,   9, 108),
            Rgb::new( 90,  24, 154),
            Rgb::new(123,  44, 191),
            Rgb::new(157,  78, 221),
            Rgb::new(199, 125, 255),
            Rgb::new(224, 170, 255)
        ];

        let fg = [
            Rgb::new(248, 249, 250),
            Rgb::new(222, 226, 230),
            Rgb::new(206, 212, 218),
            Rgb::new(173, 181, 189),
            Rgb::new(108, 117, 125),
            Rgb::new( 73,  80,  87),
            Rgb::new( 52,  58,  64),
            Rgb::new( 33,  37,  41)
        ];

        Self { bg, fg }
    }
}

