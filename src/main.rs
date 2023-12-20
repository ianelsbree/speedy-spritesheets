mod button;
mod component;

use crate::button::Button;
use crate::component::Component;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

fn print_test() {
    println!("Button test worked I think.");
}

fn close_window() {}

fn main() -> Result<(), String> {
    let screen_width = 600;
    let screen_height = 400;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Speedy Spritesheets", screen_width, screen_height)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();

    let (mut mouse_x, mut mouse_y) = (0, 0);

    let mut mouse_buttons = [0; 3];

    let screen_area = Rect::new(0, 0, screen_width, screen_height);
    let mut main_component = Component::new(screen_area, canvas, Color::RGB(52, 52, 52));

    let test_button = Button::new(
        Rect::new(50, 50, 150, 50),
        Color::WHITE,
        Color::BLACK,
        Box::new(|| print_test()),
    );
    main_component.add_button(test_button);

    while running {
        // Calculations and event handling
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { timestamp } => {
                    println!("[{0}.{1}]Event quit.", timestamp / 1000, timestamp % 1000);
                    running = false
                }
                Event::MouseMotion { x, y, .. } => {
                    mouse_x = x;
                    mouse_y = y;
                    // println!("Mouse moved: ({x}, {y})");
                }
                Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => {
                    match mouse_btn {
                        MouseButton::Left => {
                            mouse_buttons[0] = 1;
                            main_component.on_click(Point::new(x, y));
                        }
                        MouseButton::Right => mouse_buttons[1] = 1,
                        MouseButton::Middle => mouse_buttons[2] = 1,
                        _ => {}
                    }
                    /*println!(
                        "Mouse button down: {:?} mouse button at ({}, {})",
                        mouse_btn, x, y
                    )*/
                }
                Event::MouseButtonUp {
                    mouse_btn, x, y, ..
                } => match mouse_btn {
                    MouseButton::Left => mouse_buttons[0] = 0,
                    MouseButton::Right => mouse_buttons[1] = 0,
                    MouseButton::Middle => mouse_buttons[2] = 0,
                    _ => {}
                },
                _ => {}
            }
        }

        // Rendering pre-computes
        /*clear_color = Color::RGB(
            52 + 52 * mouse_buttons[0],
            52 + 52 * mouse_buttons[1],
            52 + 52 * mouse_buttons[2],
        );*/

        main_component.render()?;
    }

    Ok(())
}
