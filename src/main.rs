extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

struct Circle {
    x: i16,
    y: i16,
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Circles", 640, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // the canvas allows us to both manipulate the property of the window and to change its content
    // via hardware or software rendering. See CanvasBuilder for more info.
    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    let mut circles = Vec::<Circle>::new();

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

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
                    circles.push(Circle { x: x as i16, y: y as i16 });
                },

                _ => {},
            }
        }

        frame(&mut canvas, &circles)?;

        canvas.present();

        // std::thread::sleep(Duration::from_millis(2000));
    }

    Ok(())
}

fn frame(canvas: &mut Canvas<Window>, circles: &Vec<Circle>) -> Result<(), String> {

    for circle in circles {
        canvas.circle(circle.x, circle.y, 50, Color::RGB(0, 255, 0))?;
    }

    canvas.present();

    Ok(())
}
