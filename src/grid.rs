use druid::{Point, Rect, Size};

use crate::Square;

pub struct Grid {
    pub square: Size,
}

impl Grid {
    const N: u32 = 8;
    const M: u32 = 8;

    pub fn new(size: Size) -> Self {
        Self {
            square: Size::new(
                (size.width ) / f64::from(Self::N),
                (size.width) / f64::from(Self::M),
            ),
        }
    }

    pub fn rect(&self, sq: Square) -> Rect {
        let x = self.square.width * f64::from(sq.file);
        let y = self.square.height * f64::from(sq.rank);
        Rect::from_origin_size(Point::new(x, y), self.square)
    }

    // #[deprecated(since = "0.1.0", note = "use `rect(&self, sq: Square)` instead")]
    pub fn square(&self, i: i32, j: i32) -> Rect {
        let x = self.square.width * f64::from(i);
        let y = self.square.height * f64::from(j);
        Rect::from_origin_size(Point::new(x, y), self.square)
    }

    pub fn horz_line(&self, j: i32) -> druid::kurbo::Line {
        let x = self.square.width * f64::from(j);
        druid::kurbo::Line::new(Point::new(x, 0.0), Point::new(x, self.square.height * 8.0))
    }

    pub fn vert_line(&self, i: i32) -> druid::kurbo::Line {
        let y = self.square.height * f64::from(i);
        druid::kurbo::Line::new(Point::new(0.0, y), Point::new(self.square.width * 8.0, y))
    }

    pub fn square_from_mouse(&self, mouse: &druid::MouseEvent) -> Square {
        let (x, y) = (mouse.pos.x  / self.square.width, mouse.pos.y  / self.square.height);
        let square = Square::new(x as i32, y as i32);
        square
    }
}
