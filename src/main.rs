#![allow(non_upper_case_globals, dead_code)]
#![feature(box_syntax)]
#![feature(slice_patterns)]

mod gamestate;
use gamestate::*;

extern crate rand;
//use rand::{Rng, SeedableRng, XorShiftRng};

extern crate piston;
extern crate image;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::rc::Rc;
use std::cell::RefCell;

use opengl_graphics::{ GlGraphics, OpenGL, Texture };
use sdl2_window::Sdl2Window;
use image::GenericImage;
use piston::input::{ MouseButton };
use piston::window::{WindowSettings, AdvancedWindow};
use piston::event::*;

fn main() {
    let mut game = GameState::new();

    let mut paused = true;
    let mut raining = true;

    let mut mouse_buttons_pressed = (false, false); //left, right

    let opengl = OpenGL::_3_2;
    let (width, height) = (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);
    let window = Sdl2Window::new(
        opengl,
        WindowSettings::new("Falling Sand", (width, height))
        .exit_on_esc(true)
        //what those do?
        .vsync(true)
        .samples(4)
        .fullscreen(true)
    );
    let window = Rc::new(RefCell::new(window));
    let mut image = image::ImageBuffer::from_pixel(width, height, image::Rgba([0, 0, 0, 255]));
    let blank = image.clone().into_raw();
    let mut texture = Texture::from_image(&image);
    let ref mut gl = GlGraphics::new(opengl);
    for e in window.clone().events().max_fps(120).ups(60) {
        use piston::event::*;
        use piston::input::{Button, Key};

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Space => {
                    paused = !paused;
                    window.borrow_mut().set_capture_cursor(!paused); //remove cursor when game is running
                },
                Key::R => {raining = !raining},
                _ => {}
            }
        }

        if let Some(_) = e.update_args() {
            //Update game and image on every loop
            if !paused {
                if raining {game.rain()}
                game.update();
            }//if !paused {
        }

        if paused {
            if let Some(button) = e.press_args() {
                match button {
                    Button::Mouse(MouseButton::Left) => {mouse_buttons_pressed.0 = true},
                    Button::Mouse(MouseButton::Right) => {mouse_buttons_pressed.1 = true},
                    _ => {}
                }
            }
            if let Some(button) = e.release_args() {
                match button {
                    Button::Mouse(MouseButton::Left) => {mouse_buttons_pressed.0 = false},
                    Button::Mouse(MouseButton::Right) => {mouse_buttons_pressed.1 = false},
                    _ => {}
                }
            }
            if let Some(pos) = e.mouse_cursor_args() {
                //get mouse coordinates and draw something if there is place
                let (mouse_x, mouse_y) = (pos[0] as i16, pos[1] as i16);

                match mouse_buttons_pressed {
                   (true, false) => {
                       game.paint_square_obstacles(mouse_x, mouse_y, 30, 30);
                   }
                   (false, true) => {
                       //remove everything from map and both lists
                       game.remove_square(mouse_x, mouse_y, 30, 30);
                   }
                   _ => {}
               }
            }
        } //if paused

        if let Some(args) = e.render_args() {

            image = image::ImageBuffer::from_vec(width, height, blank.clone()).unwrap();
            for particle in game.particles.iter() {
                image.put_pixel(particle.x as u32, particle.y as u32, image::Rgba([238,232,170,255]));
            }
            for obstacle in game.obstacles.iter() {
                image.put_pixel(obstacle.x as u32, obstacle.y as u32, image::Rgba([128,0,0,255]));
            }
            texture.update(&image);
            gl.draw(args.viewport(), |c, gl| {
                graphics::clear([1.0; 4], gl);
                graphics::image(&texture, c.transform, gl);
            });

        };
    }
}
