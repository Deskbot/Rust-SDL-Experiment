extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Point;

struct Drawer {
    canvas: Canvas<Window>,
    cursor: Point,
    shape: Vec<Point>,
}

impl Drawer {
    pub fn new(canvas: Canvas<Window>) -> Drawer {
        Drawer {
            canvas,
            cursor: Point::new(0,0),
            shape: vec![],
        }
    }

    pub fn update_frame(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        for circle in &self.shape {
            self.canvas.circle(circle.x() as i16, circle.y() as i16, 50, Color::RGB(0, 255, 0))?;
        }

        self.canvas.set_draw_color(Color::RGB(0, 0, 255));
        self.canvas.draw_lines(self.shape.as_ref())?;

        self.canvas.present();

        Ok(())
    }

    pub fn add_point(&mut self, point: Point) {
        self.shape.push(point);
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Circles", 640, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

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

    let mut drawer = Drawer::new(canvas);

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
                    drawer.add_point(Point::new(x, y));
                },

                _ => {},
            }
        }

        drawer.update_frame()?;

        // std::thread::sleep(Duration::from_millis(2000));
    }

    Ok(())
}

