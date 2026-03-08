use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use chrono::Utc;
use chrono_tz::America::New_York;

use pulse_nyct::service::{Service, Services};
use pulse_nyct::train::{Direction, arrival_pairs_by_name_in_minutes};

use crate::bullets::BulletDisplay;
use crate::matrix::{Bounds, Draw, Point, setup_adafruit_led_matrix_32x64};
use crate::text::create_standard_4x6_text;

fn draw_incoming(
    pair: (Service, i64),
    canvas: &mut rpi_led_matrix::LedCanvas,
    point: &Point,
    bounds: &Bounds,
) {
    pair.0.bullet_small().draw(canvas, point, bounds);

    let string = match pair.1 {
        0 => "Now".to_string(),
        m if m < 10 => format!(" {}M", m),
        m => format!("{}M", m),
    };

    let text = create_standard_4x6_text(&string);

    let bullet_height = 8;
    let text_point = Point {
        x: point.x - 3,
        y: point.y + bullet_height + 2,
    };
    text.draw(canvas, &text_point, bounds);
}

pub fn draw_next_three(station: String, services: Services, direction: Direction) {
    let matrix = setup_adafruit_led_matrix_32x64();
    let mut canvas = matrix.offscreen_canvas();
    let default_bounds = Bounds::default();

    let station_text = create_standard_4x6_text(&station);
    let mut station_point = Point {
        x: default_bounds.right,
        y: 1,
    };

    let time_point = Point { x: 3, y: 8 };

    let (tx, rx) = mpsc::channel();

    let station_clone = station.clone();
    let services_clone = services.clone();
    thread::spawn(move || {
        loop {
            let pairs =
                arrival_pairs_by_name_in_minutes(&station_clone, &services_clone, direction);
            tx.send(pairs).ok();
            thread::sleep(Duration::from_secs(30));
        }
    });

    let mut pairs = arrival_pairs_by_name_in_minutes(&station, &services, direction);

    loop {
        canvas.clear();

        station_text.draw(&mut canvas, &station_point, &default_bounds);
        station_point.x -= 1;
        if station_point.x < -station_text.width() {
            station_point.x = default_bounds.right;
        }

        let time = Utc::now().with_timezone(&New_York);

        let time_string = time.format("%H:%M:%S %Z").to_string();
        let time_text = create_standard_4x6_text(&time_string);
        time_text.draw(&mut canvas, &time_point, &default_bounds);

        if let Ok(data) = rx.try_recv() {
            pairs = data;
        }

        let relative_x: i32 = 6;
        let relative_y: i32 = 15;
        let width: i32 = 21;

        draw_incoming(
            pairs[0],
            &mut canvas,
            &Point {
                x: relative_x,
                y: relative_y,
            },
            &default_bounds,
        );
        draw_incoming(
            pairs[1],
            &mut canvas,
            &Point {
                x: relative_x + 1 * width,
                y: relative_y,
            },
            &default_bounds,
        );
        draw_incoming(
            pairs[2],
            &mut canvas,
            &Point {
                x: relative_x + 2 * width,
                y: relative_y,
            },
            &default_bounds,
        );

        canvas = matrix.swap(canvas);
        thread::sleep(Duration::from_millis(40));
    }
}
