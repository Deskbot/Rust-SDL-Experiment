use std::{fmt, iter::FromIterator, ops::Add};

use sdl2::rect::Point;

use crate::{grid::Grid, view::View};

pub struct Model {
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

    pub fn to_svg(&self) -> String {
        let opening = "<svg viewBox=\"0 0 200 100\" xmlns=\"http://www.w3.org/2000/svg\">";

        let mut points = String::new();

        for shape in &self.shape {
            points = format!("{}{},{} ", points, shape.x(), shape.y());
        }

        let polygon = format!("<polygon points=\"{}\" fill=\"none\" stroke=\"black\" />", points);
        let closing = "</svg>";

        return format!("{}{}{}", opening, polygon, closing);
    }

    pub fn update(&mut self) -> Result<(), String> {
        self.view.reset();
        self.update_frame()?;
        self.view.update();

        Ok(())
    }

    fn update_frame(&mut self) -> Result<(), String> {
        // grid
        self.view.grid(&self.grid)?;

        // lines
        self.view.shape(self.shape.as_ref())?;

        // circles
        for circle in &self.shape {
            self.view.circle(circle)?;
        }

        // circle at cursor
        self.view.cursor_circle(&self.cursor)?;

        Ok(())
    }
}
