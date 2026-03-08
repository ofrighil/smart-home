// mod bullets_16;
mod bullets_8;

use rpi_led_matrix::LedColor;

use pulse_nyct::service::Service;

use crate::matrix::{BLUE, BitTest, Bounds, Draw, ORANGE, Point, WHITE};

pub struct Bullet<T: 'static> {
    representation: &'static [T],
    color: &'static LedColor,
}

impl Draw for Bullet<u8> {
    fn draw(&self, canvas: &mut rpi_led_matrix::LedCanvas, point: &Point, _bounds: &Bounds) {
        for row in 0..8 {
            let circle_bits = bullets_8::BACKGROUND[row];
            let letter_bits = self.representation[row];

            for col in (0..8).filter(|col| circle_bits.is_set(7 - col)) {
                let x = point.x + (col as i32);
                let y = point.y + (row as i32);

                if letter_bits.is_set(7 - col) {
                    canvas.set(x, y, &WHITE);
                } else {
                    canvas.set(x, y, self.color);
                }
            }
        }
    }
}

// impl Draw for Bullet<u16> {
//     fn draw(&self, canvas: &mut rpi_led_matrix::LedCanvas, point: &Point, _bounds: &Bounds) {
//         for row in 0..16 {
//             let circle_bits = bullets_16::BACKGROUND[row];
//             let letter_bits = self.representation[row];
// 
//             for col in (0..16).filter(|col| circle_bits.is_set(15 - col)) {
//                 let x = point.x + (col as i32);
//                 let y = point.y + (row as i32);
// 
//                 if letter_bits.is_set(15 - col) {
//                     canvas.set(x, y, &WHITE);
//                 } else {
//                     canvas.set(x, y, self.color);
//                 }
//             }
//         }
//     }
// }

pub const BULLET_A_SMALL: Bullet<u8> = Bullet {
    representation: &bullets_8::SERVICE_A,
    color: &BLUE,
};

pub const BULLET_C_SMALL: Bullet<u8> = Bullet {
    representation: &bullets_8::SERVICE_C,
    color: &BLUE,
};

pub const BULLET_E_SMALL: Bullet<u8> = Bullet {
    representation: &bullets_8::SERVICE_E,
    color: &BLUE,
};

pub const BULLET_B_SMALL: Bullet<u8> = Bullet {
    representation: &bullets_8::SERVICE_B,
    color: &ORANGE,
};

pub const BULLET_D_SMALL: Bullet<u8> = Bullet {
    representation: &bullets_8::SERVICE_D,
    color: &ORANGE,
};

pub const BULLET_F_SMALL: Bullet<u8> = Bullet {
    representation: &bullets_8::SERVICE_F,
    color: &ORANGE,
};

pub const BULLET_M_SMALL: Bullet<u8> = Bullet {
    representation: &bullets_8::SERVICE_M,
    color: &ORANGE,
};

// pub const BULLET_A_LARGE: Bullet<u16> = Bullet {
//     representation: &bullets_16::SERVICE_A,
//     color: &BLUE,
// };
// 
// pub const BULLET_C_LARGE: Bullet<u16> = Bullet {
//     representation: &bullets_16::SERVICE_C,
//     color: &BLUE,
// };
// 
// pub const BULLET_E_LARGE: Bullet<u16> = Bullet {
//     representation: &bullets_16::SERVICE_E,
//     color: &BLUE,
// };
// 
// pub const BULLET_B_LARGE: Bullet<u16> = Bullet {
//     representation: &bullets_16::SERVICE_B,
//     color: &ORANGE,
// };
// 
// pub const BULLET_D_LARGE: Bullet<u16> = Bullet {
//     representation: &bullets_16::SERVICE_D,
//     color: &ORANGE,
// };
// 
// pub const BULLET_F_LARGE: Bullet<u16> = Bullet {
//     representation: &bullets_16::SERVICE_F,
//     color: &ORANGE,
// };
// 
// pub const BULLET_M_LARGE: Bullet<u16> = Bullet {
//     representation: &bullets_16::SERVICE_M,
//     color: &ORANGE,
// };

pub trait BulletDisplay {
    fn bullet_small(&self) -> &Bullet<u8>;
    // fn bullet_large(&self) -> &Bullet<u16>;
}

impl BulletDisplay for Service {
    fn bullet_small(&self) -> &Bullet<u8> {
        match self {
            Service::A => &BULLET_A_SMALL,
            Service::C => &BULLET_C_SMALL,
            Service::E => &BULLET_E_SMALL,
            Service::B => &BULLET_B_SMALL,
            Service::D => &BULLET_D_SMALL,
            Service::F => &BULLET_F_SMALL,
            Service::M => &BULLET_M_SMALL,
            _ => &BULLET_M_SMALL,
        }
    }

    // fn bullet_large(&self) -> &Bullet<u16> {
    //     match self {
    //         Service::A => &BULLET_A_LARGE,
    //         Service::C => &BULLET_C_LARGE,
    //         Service::E => &BULLET_E_LARGE,
    //         Service::B => &BULLET_B_LARGE,
    //         Service::D => &BULLET_D_LARGE,
    //         Service::F => &BULLET_F_LARGE,
    //         Service::M => &BULLET_M_LARGE,
    //         _ => &BULLET_M_LARGE,
    //     }
    // }
}
