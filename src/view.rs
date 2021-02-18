use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Point;

use crate::grid::{Grid, Line};

pub struct View {
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

    pub fn cursor_circle(&mut self, point: &Point) -> Result<(), String> {
        self.canvas.circle(point.x() as i16, point.y() as i16, 20, Color::RGB(100, 100, 100))?;
        Ok(())
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
