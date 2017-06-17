#![allow(non_upper_case_globals, dead_code)]

mod gamestate;
use gamestate::*;

extern crate rand;
extern crate piston_window;
extern crate image as im;

use piston_window::*;
use std::borrow::BorrowMut;
use im::Pixel;

fn main() {
    let mut game = GameState::new();

    let mut paused = true;
    let mut raining = true;
    let mut redraw_needed = true;

    let mut mouse_buttons_pressed = (false, false); //left, right

    let (width, height) = (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);

    let mut window: PistonWindow =
        WindowSettings::new("Falling Sand", (width, height))
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let pixel = im::Rgba::from_channels(0, 0, 0, 255);
    let mut canvas = im::ImageBuffer::from_pixel(width, height, pixel);
    let blank = canvas.clone().into_raw();
    let tex_set = &TextureSettings::new();
    // https://github.com/PistonDevelopers/piston-examples/blob/master/src/paint.rs
    let mut texture = Texture::from_image(&mut *window.factory.borrow_mut(), &canvas, tex_set)
        .unwrap();

    // for e in window.clone().events().max_fps(120).ups(60) {
    while let Some(e) = window.next() {

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Space => {
                    paused = !paused;
                    // window.borrow_mut().set_capture_cursor(!paused); //remove cursor when game is running
                }
                Key::D => {
                    println!("{:?}", game);
                }
                Key::R => raining = !raining,
                _ => {}
            }
        }

        if let Some(_) = e.update_args() {
            // Update game and image on every loop
            if !paused {
                if raining {
                    game.rain()
                }
                game.update();
                redraw_needed = true;
            } //if !paused {
        }

        if paused {
            if let Some(button) = e.press_args() {
                match button {
                    Button::Mouse(MouseButton::Left) => mouse_buttons_pressed.0 = true,
                    Button::Mouse(MouseButton::Right) => mouse_buttons_pressed.1 = true,
                    _ => {}
                }
            }
            if let Some(button) = e.release_args() {
                match button {
                    Button::Mouse(MouseButton::Left) => mouse_buttons_pressed.0 = false,
                    Button::Mouse(MouseButton::Right) => mouse_buttons_pressed.1 = false,
                    _ => {}
                }
            }
            if let Some(pos) = e.mouse_cursor_args() {
                redraw_needed = true;
                // get mouse coordinates and draw something if there is place
                let (mouse_x, mouse_y) = (pos[0] as i16, pos[1] as i16);

                match mouse_buttons_pressed {
                    (true, false) => {
                        game.paint_square_obstacles(mouse_x, mouse_y, 30, 30);
                    }
                    (false, true) => {
                        // remove everything from map and both lists
                        game.remove_square(mouse_x, mouse_y, 30, 30);
                    }
                    _ => {}
                }
            }
        } //if paused

        if redraw_needed {
            if let Some(_) = e.render_args() {
                canvas = im::ImageBuffer::from_vec(width, height, blank.clone()).unwrap();
                for particle in game.particles.iter() {
                    canvas.put_pixel(
                        particle.x as u32,
                        particle.y as u32,
                        im::Rgba([238, 232, 170, 255]),
                    );
                }
                for obstacle in game.obstacles.iter() {
                    canvas.put_pixel(
                        obstacle.x as u32,
                        obstacle.y as u32,
                        im::Rgba([128, 0, 0, 255]),
                    );
                }
                texture.update(&mut window.encoder, &canvas).unwrap();
                window.draw_2d(&e, |c, g| { image(&texture, c.transform, g); });
                redraw_needed = false;
            }
        };
    }
}
