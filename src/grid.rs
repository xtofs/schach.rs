use druid::{Point, Rect, Size};

use crate::Square;

pub(crate) struct Grid {
    cell: Size,
}

impl Grid {
    pub fn new(size: Size, n: u32, m: u32) -> Self {
        Self {
            cell: Size::new(size.width / f64::from(n), size.width / f64::from(m)),
        }
    }

    pub fn rect(&self, sq: Square) -> Rect {
        let x = self.cell.width * f64::from(sq.file);
        let y = self.cell.height * f64::from(sq.rank);
        Rect::from_origin_size(Point::new(x, y), self.cell)
    }

    // #[deprecated(since = "0.1.0", note = "please use `rect` instead")]
    pub fn square(&self, i: i32, j: i32) -> Rect {
        let x = self.cell.width * f64::from(i);
        let y = self.cell.height * f64::from(j);
        Rect::from_origin_size(Point::new(x, y), self.cell)
    }

    pub fn horz_line(&self, j: i32) -> druid::kurbo::Line {
        let x = self.cell.width * f64::from(j);
        druid::kurbo::Line::new(Point::new(x, 0.0), Point::new(x, self.cell.height))
    }

    pub fn vert_line(&self, i: i32) -> druid::kurbo::Line {
        let y = self.cell.height * f64::from(i);
        druid::kurbo::Line::new(Point::new(0.0, y), Point::new(self.cell.width, y))
    }
}
