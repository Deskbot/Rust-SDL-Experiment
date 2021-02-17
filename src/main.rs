use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

extern crate sdl2;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video = sdl_context.video()?;

    let window = video.window("SDL Learning", 640, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    let mut tick = 0;

    println!("This example simply prints all events SDL knows about.");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                // skip mouse motion intentionally because of the verbose it might cause.
                Event::MouseMotion { .. } => {}
                e => {
                    println!("{:?}", e);
                }
            }
        }

        {
            // Update the window title.
            let window = canvas.window_mut();

            let position = window.position();
            let size = window.size();
            let title = format!(
                "Window - pos({}x{}), size({}x{}): {}",
                position.0, position.1, size.0, size.1, tick
            );
            window.set_title(&title).map_err(|e| e.to_string())?;

            tick += 1;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
