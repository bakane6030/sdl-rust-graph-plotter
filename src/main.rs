mod drawing;

use crate::drawing::CanvasExt;
use glam::{vec2, Vec2};
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::EventPump;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl3 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let mut mouse_positions = vec![mouse_pos(&event_pump)];

    loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return;
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        mouse_positions.push(mouse_pos(&event_pump));
        if mouse_positions.len() > 100 {
            mouse_positions = mouse_positions[mouse_positions.len() - 100..].to_vec();
        }

        canvas
            .draw_thick_lines(&mouse_positions, 5.0, Color::RGB(255, 255, 255))
            .unwrap();

        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 144));
    }
}

fn mouse_pos(event_pump: &EventPump) -> Vec2 {
    let mouse_state = event_pump.mouse_state();
    vec2(mouse_state.x(), mouse_state.y())
}
