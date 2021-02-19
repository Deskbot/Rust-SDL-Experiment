extern crate sdl2;

mod grid;
mod model;
mod view;

use std::fs::File;
use std::io::prelude::*;

use nfd::Response;
use sdl2::event::Event;
use sdl2::image::LoadSurface;
use sdl2::keyboard::Mod;
use sdl2::surface::Surface;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use model::Model;
use view::View;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let width = 640;
    let height = 480;

    let mut window = video_subsystem
        .window("Ramiel", 640, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // show logo

    let logo: Surface = LoadSurface::from_file("./assets/logo.png")?;
    window.set_icon(logo);

    // turn window into canvas

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    // init canvas

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut model = Model::new(View::new(canvas, width, height));

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                    => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. }
                    => break 'running,

                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    model.add_point(Point::new(x, y));
                },

                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Right,
                    x,
                    y,
                    ..
                } => {
                    let point_to_remove =  model.get_vertex_near(&Point::new(x, y))
                        .map(Point::clone);
                    // don't hold a reference to the point in the model
                    // because we need to mutate the model

                    match point_to_remove {
                        None => {},
                        Some(vertex) => {
                            model.delete_vertex(&vertex);
                        }
                    }
                },

                Event::MouseMotion {
                    x,
                    y,
                    ..
                } => {
                    let cursor_pos = Point::new(x,y);

                    if let Some(existing_vertex) = model.get_vertex_near(&cursor_pos) {
                        model.highlight(existing_vertex.clone());
                    }

                    model.set_cursor(Point::new(x,y));
                },

                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    keymod,
                    ..
                } => {
                    if keymod.contains(Mod::LCTRLMOD) || keymod.contains(Mod::RCTRLMOD) {
                        let svg = model.to_svg();
                        println!("{}", svg);
                        save_to_file(&svg);
                    }
                },

                _ => {},
            }
        }

        model.update()?;

        // std::thread::sleep(Duration::from_millis(2000));
    }

    Ok(())
}

fn save_to_file(s: &String) {
    let result = nfd::open_save_dialog(None, None);

    if let Ok(Response::Okay(file_path)) = result {
        println!("Saving to: {}", file_path);
        println!("File content: {}", s);

        let result = File::create(file_path)
            .and_then(|mut file| file.write_all(s.as_bytes()));

        if result.is_err() {
            println!("Could not write file.");
        }
    }
}
