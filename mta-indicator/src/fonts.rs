mod font4x6;
mod font5x7;
mod font6x10;

use rpi_led_matrix::{LedCanvas, LedColor};

use crate::matrix::{BitTest, Bounds, Draw, Point};

pub struct Font {
    width: i32,
    // height: i32,
    spacing: i32,
    lookup: fn(char) -> &'static [u8],
}

impl Font {
    fn advance(&self) -> i32 {
        self.width + self.spacing
    }
}

struct Character<T: 'static> {
    representation: &'static [T],
    width: i32,
    color: &'static LedColor,
}

impl<T: BitTest> Draw for Character<T> {
    fn draw(&self, canvas: &mut LedCanvas, point: &Point, bounds: &Bounds) {
        let bit_shift = self.width - 1;
        for (row, &bits) in self.representation.iter().enumerate() {
            for dot in (0..self.width)
                .filter(|col| bits.is_set(bit_shift - col))
                .map(|col| Point {
                    x: point.x + col,
                    y: point.y + row as i32,
                })
                .filter(|dot| bounds.in_bounds(&dot))
            {
                canvas.set(dot.x, dot.y, self.color);
            }
        }
    }
}

pub struct Text<'a> {
    pub string: &'a str,
    pub font: &'static Font,
    pub color: &'static LedColor,
}

struct TextIter<'a> {
    chars: std::str::Chars<'a>,
    font: &'static Font,
    color: &'static LedColor,
}

impl<'a> Iterator for TextIter<'a> {
    type Item = Character<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next().map(|ch| Character {
            representation: (self.font.lookup)(ch),
            width: self.font.width,
            color: self.color,
        })
    }
}

impl<'a> Text<'a> {
    pub fn width(&self) -> i32 {
        let length = self.string.len() as i32;
        if length == 0 {
            0
        } else {
            length * self.font.advance() - self.font.spacing
        }
    }

    fn iter(&self) -> TextIter<'_> {
        TextIter {
            chars: self.string.chars(),
            font: self.font,
            color: self.color,
        }
    }
}

impl<'a> Draw for Text<'a> {
    fn draw(&self, canvas: &mut LedCanvas, point: &Point, bounds: &Bounds) {
        let char_advance = self.font.advance();
        let mut cursor_x = point.x;

        for character in self.iter() {
            if cursor_x + self.font.width < bounds.left {
                cursor_x += char_advance;
                continue;
            }

            if cursor_x >= bounds.right {
                break;
            }

            character.draw(
                canvas,
                &Point {
                    x: cursor_x,
                    y: point.y,
                },
                &bounds,
            );

            cursor_x += char_advance;
        }
    }
}

pub const FONT_4X6: Font = Font {
    width: 4,
    // height: 6,
    spacing: 1,
    lookup: |ch| font4x6::get_char(ch),
};

pub const FONT_5X7: Font = Font {
    width: 5,
    // height: 7,
    spacing: 1,
    lookup: |ch| font5x7::get_char(ch),
};

pub const FONT_6X10: Font = Font {
    width: 6,
    // height: 7,
    spacing: 1,
    lookup: |ch| font6x10::get_char(ch),
};
