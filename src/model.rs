use sdl2::rect::Point;

use crate::{grid::Grid, view::{self, View}};

pub struct Model {
    cursor: Point,
    dragging: Option<Point>,
    grid: Grid,
    highlighted_vertex: Option<Point>,
    shape: Vec<Point>,
    view: View,
}

impl Model {
    pub fn new(view: View) -> Model {
        Model {
            view,
            cursor: Point::new(0,0),
            dragging: None,
            grid: Grid { size: 40 },
            highlighted_vertex: None,
            shape: vec![],
        }
    }

    pub fn add_point(&mut self, point: Point) {
        self.shape.push(self.grid.nearest_vertex(&point));
    }

    pub fn delete_vertex(&mut self, vertex: &Point) -> Option<()> {
        let pos = self.shape.iter().position(|x| *x == *vertex)?;
        self.shape.remove(pos);
        Some(())
    }

    pub fn drag_to_point_near(&mut self, point: &Point) {
        // find the vertex being dragged
        if let Some(dragging) = self.dragging {
            let maybe_index = self.shape.iter().position(|&vertex| vertex == dragging);

            let destination = self.grid.nearest_vertex(point);

            match maybe_index {
                None => {},
                Some(index) => {
                    self.shape[index] = destination;
                    self.dragging = Some(destination);
                },
            }
        }
    }

    pub fn get_vertex_near(&self, point: &Point) -> Option<&Point> {
        let mut best_vertex = None;
        let mut best_distance = None;

        for vertex in &self.shape {
            let dist_squared = vertex.relative_distance(&point);

            if dist_squared > view::vertex_size.pow(2) as i32 {
                continue;
            }

            if let None = best_distance {
                best_distance = Some(dist_squared);
                best_vertex = Some(vertex);
            } else if dist_squared < best_distance.unwrap() {
                best_distance = Some(dist_squared);
                best_vertex = Some(vertex);
            }
        }

        return best_vertex;
    }

    pub fn highlight(&mut self, point: Point) {
        self.highlighted_vertex = Some(point);
    }

    pub fn is_dragging(&self) -> bool {
        return self.dragging.is_some();
    }

    pub fn set_cursor(&mut self, point: Point) {
        self.cursor = self.grid.nearest_vertex(&point);
    }

    pub fn start_dragging(&mut self, point: Point) {
        self.dragging = Some(point);
    }

    pub fn stop_dragging(&mut self) {
        self.dragging = None;
    }

    pub fn to_svg(&self) -> String {
        let opening = format!(
            "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
            self.view.width,
            self.view.height
        );

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
        if self.highlighted_vertex.map_or(false, |highlighted| highlighted == self.cursor) {
            self.view.highlighted_circle(&self.cursor)?;
        } else {
            self.view.cursor_circle(&self.cursor)?;
        }

        Ok(())
    }
}

trait Geom {
    fn relative_distance(&self, other: &Self) -> i32;
}

impl Geom for Point {
    fn relative_distance(&self, other: &Self) -> i32 {
        return (self.x() - other.x()).pow(2)
            + (self.y() - other.y()).pow(2);
    }
}
