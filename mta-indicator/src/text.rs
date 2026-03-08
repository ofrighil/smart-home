use rpi_led_matrix::{LedCanvas, LedColor};

use crate::fonts::{Font, FONT_4X6};
use crate::matrix::{BitTest, Bounds, Draw, Point, WHITE};

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

    pub fn height(&self) -> i32 {
        self.font.height
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

pub fn create_standard_4x6_text(string: &str) -> Text<'_> {
    Text {
        string: &string,
        font: &FONT_4X6,
        color: &WHITE,
    }
}
