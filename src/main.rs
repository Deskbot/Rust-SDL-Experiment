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

struct Model {
    cursor: Point,
    shape: Vec<Point>,
    view: View,
}

impl Model {
    pub fn new(view: View) -> Model {
        Model {
            view,
            cursor: Point::new(0,0),
            shape: vec![],
        }
    }

    pub fn update_frame(&mut self) -> Result<(), String> {
        // circles

        for circle in &self.shape {
            self.view.circle(circle)?;
        }

        // lines

        self.view.lines(self.shape.as_ref())?;

        // circle at cursor

        self.view.circle(&self.cursor)?;

        Ok(())
    }

    pub fn add_point(&mut self, point: Point) {
        self.shape.push(point);
    }

    pub fn set_cursor(&mut self, point: Point) {
        self.cursor = point;
    }

    pub fn update(&mut self) -> Result<(), String> {
        self.view.reset();
        self.update_frame()?;
        self.view.update();

        Ok(())
    }
}

struct View {
    canvas: Canvas<Window>,
}

impl View {
    pub fn new(canvas: Canvas<Window>) -> View {
        View {
            canvas,
        }
    }

    pub fn circle(&mut self, point: &Point) -> Result<(), String> {
        self.canvas.circle(point.x() as i16, point.y() as i16, 50, Color::RGB(0, 255, 0))?;
        Ok(())
    }

    pub fn lines(&mut self, points: &[Point]) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(0, 255, 0));
        self.canvas.draw_lines(points)?;
        Ok(())
    }

    pub fn reset(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    pub fn update(&mut self) {
        self.canvas.present();
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

    let mut model = Model::new(View::new(canvas));

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

                Event::MouseMotion {
                    x,
                    y,
                    ..
                } => {
                    model.set_cursor(Point::new(x,y));
                }

                _ => {},
            }
        }

        model.update();

        // std::thread::sleep(Duration::from_millis(2000));
    }

    Ok(())
}

