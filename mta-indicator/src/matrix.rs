use rpi_led_matrix::{LedCanvas, LedColor, LedMatrix, LedMatrixOptions, LedRuntimeOptions};

pub const BLUE: LedColor = LedColor {
    red: 0,
    green: 57,
    blue: 166,
};

pub const BROWN: LedColor = LedColor {
    red: 153,
    green: 102,
    blue: 51,
};

pub const GRAY_DARK: LedColor = LedColor {
    red: 128,
    green: 129,
    blue: 131,
};

pub const GRAY_LIGHT: LedColor = LedColor {
    red: 167,
    green: 169,
    blue: 172,
};

pub const GREEN: LedColor = LedColor {
    red: 0,
    green: 147,
    blue: 60,
};

pub const LIME: LedColor = LedColor {
    red: 108,
    green: 190,
    blue: 69,
};

pub const ORANGE: LedColor = LedColor {
    red: 255,
    green: 99,
    blue: 25,
};

pub const PURPLE: LedColor = LedColor {
    red: 185,
    green: 51,
    blue: 173,
};

pub const RED: LedColor = LedColor {
    red: 238,
    green: 53,
    blue: 46,
};

pub const TURQUOISE: LedColor = LedColor {
    red: 0,
    green: 173,
    blue: 208,
};

pub const WHITE: LedColor = LedColor {
    red: 255,
    green: 255,
    blue: 255,
};

pub const YELLOW: LedColor = LedColor {
    red: 252,
    green: 204,
    blue: 10,
};

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Bounds {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            left: 0,
            right: 64,
            bottom: 0,
            top: 32,
        }
    }
}

impl Bounds {
    pub fn in_bounds(&self, point: &Point) -> bool {
        if self.left <= point.x
            && point.x < self.right
            && self.bottom <= point.y
            && point.y < self.top
        {
            true
        } else {
            false
        }
    }
}

pub trait BitTest: Copy + 'static {
    fn is_set(self, bit: i32) -> bool;
}

impl BitTest for u8 {
    fn is_set(self, bit: i32) -> bool {
        (self >> bit) & 1 == 1
    }
}

impl BitTest for u16 {
    fn is_set(self, bit: i32) -> bool {
        (self >> bit) & 1 == 1
    }
}

pub trait Draw {
    fn draw(&self, canvas: &mut LedCanvas, point: &Point, bounds: &Bounds);
}

pub fn setup_adafruit_led_matrix_32x64() -> LedMatrix {
    let mut options = LedMatrixOptions::new();
    options.set_rows(32);
    options.set_cols(64);
    options.set_hardware_mapping("adafruit-hat");
    options.set_chain_length(1);
    options.set_parallel(1);
    options.set_brightness(50).unwrap();

    let mut rt_options = LedRuntimeOptions::new();
    rt_options.set_gpio_slowdown(2);

    LedMatrix::new(Some(options), Some(rt_options)).expect("Failed to initialize LED matrix")
}
