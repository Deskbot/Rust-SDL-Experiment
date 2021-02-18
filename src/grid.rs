use sdl2::rect::Point;

pub struct Line {
    pub start: Point,
    pub end: Point,
}

pub struct Grid {
    pub size: i32,
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
