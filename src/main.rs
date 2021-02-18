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

struct Line {
    start: Point,
    end: Point,
}

struct Grid {
    size: i32,
}

impl Grid {
    pub fn lines(&self, max_width: i32, max_height: i32) -> Vec<Line> {
        let mut result = vec![];

        // vertical lines
        for i in (0..max_width).step_by(self.size as usize) {
            result.push(Line {
                start: Point::new(i, 0),
                end: Point::new(i, max_height),
            });
        }

        // horizontal lines
        for j in (0..max_height).step_by(self.size as usize) {
            result.push(Line {
                start: Point::new(0, j),
                end: Point::new(max_width, j),
            });
        }

        return result;
    }

    pub fn nearest_vertex(&self, point: &Point) -> Point {
        return Point::new(self.nearest(point.x()), self.nearest(point.y()));
    }

    fn nearest(&self, i: i32) -> i32 {
        let div = i / self.size;
        let rem = i % self.size;

        if rem < self.size / 2 {
            return div * self.size;
        } else {
            return (div + 1) * self.size;
        }
    }
}

struct Model {
    cursor: Point,
    grid: Grid,
    shape: Vec<Point>,
    view: View,
}

impl Model {
    pub fn new(view: View) -> Model {
        Model {
            view,
            cursor: Point::new(0,0),
            grid: Grid { size: 40 },
            shape: vec![],
        }
    }

    pub fn add_point(&mut self, point: Point) {
        self.shape.push(self.grid.nearest_vertex(&point));
    }

    pub fn set_cursor(&mut self, point: Point) {
        self.cursor = self.grid.nearest_vertex(&point);
    }

    pub fn update(&mut self) -> Result<(), String> {
        self.view.reset();
        self.update_frame()?;
        self.view.update();

        Ok(())
    }

    fn update_frame(&mut self) -> Result<(), String> {
        // circles

        for circle in &self.shape {
            self.view.circle(circle)?;
        }

        // lines

        self.view.lines(self.shape.as_ref())?;

        // circle at cursor

        self.view.circle(&self.cursor)?;

        // grid

        self.view.grid(&self.grid)?;

        Ok(())
    }
}

struct View {
    canvas: Canvas<Window>,
    height: i32,
    width: i32,
}

impl View {
    pub fn new(canvas: Canvas<Window>, width: i32, height: i32) -> View {
        View {
            canvas,
            height,
            width,
        }
    }

    pub fn circle(&mut self, point: &Point) -> Result<(), String> {
        self.canvas.circle(point.x() as i16, point.y() as i16, 20, Color::RGB(0, 255, 0))?;
        Ok(())
    }

    pub fn grid(&mut self, grid: &Grid) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(100, 100, 100));

        for line in grid.lines(self.width, self.height) {
            let Line { start, end } = line;
            self.canvas.draw_line(start, end)?;
        }

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

    let width = 640;
    let height = 480;

    let window = video_subsystem
        .window("Ramiel", 640, 480)
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

        model.update()?;

        // std::thread::sleep(Duration::from_millis(2000));
    }

    Ok(())
}

