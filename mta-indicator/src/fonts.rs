mod font4x6;
mod font5x7;
mod font6x10;

pub struct Font {
    pub width: i32,
    pub height: i32,
    pub spacing: i32,
    pub lookup: fn(char) -> &'static [u8],
}

impl Font {
    pub fn advance(&self) -> i32 {
        self.width + self.spacing
    }
}

pub const FONT_4X6: Font = Font {
    width: 4,
    height: 6,
    spacing: 1,
    lookup: |ch| font4x6::get_char(ch),
};

pub const FONT_5X7: Font = Font {
    width: 5,
    height: 7,
    spacing: 1,
    lookup: |ch| font5x7::get_char(ch),
};

pub const FONT_6X10: Font = Font {
    width: 6,
    height: 7,
    spacing: 1,
    lookup: |ch| font6x10::get_char(ch),
};
