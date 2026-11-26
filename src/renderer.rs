use crate::racer;
use crate::track;

use macroquad::prelude::*;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Simulation".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

pub fn draw_background() {
    let w = screen_width();
    let h = screen_height();

    clear_background(GREEN);

    draw_rectangle(
        0.0,
        h * 0.25,
        w,
        h * 0.5,
        GRAY,
    );
}

pub(crate) fn draw_kart(racer: &racer::Racer) {
    let position = racer.position();
    let center = vec2(
        48.0 + position.x as f32,
        screen_height() / 2.0 - position.y as f32,
    );

    let (sin, cos) = racer.heading().sin_cos();
    let forward = vec2(cos as f32, -sin as f32);
    let across = vec2(-forward.y, forward.x);
    let point = |x: f32, y: f32| center + forward * x + across * y;

    let rectangle = |x: f32, y: f32, w: f32, h: f32, color: Color| {
        let a = point(x, y);
        let b = point(x + w, y);
        let c = point(x + w, y + h);
        let d = point(x, y + h);
        draw_triangle(a, b, c, color);
        draw_triangle(a, c, d, color);
    };

    // An odd multiplier gives distinct RGB colors for IDs 0..2^24.
    let rgb = racer.id().wrapping_mul(0x9E37_79B1) ^ 0xD9_4632;
    let chassis = Color::from_rgba((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8, 255);

    for x in [-16.0, 6.0] {
        rectangle(x, -14.0, 10.0, 6.0, BLACK);
        rectangle(x, 8.0, 10.0, 6.0, BLACK);
    }

    rectangle(-18.0, -9.0, 36.0, 18.0, chassis);
    let seat = point(-3.0, 0.0);
    draw_circle(seat.x, seat.y, 5.0, DARKGRAY);
}

