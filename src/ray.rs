use crate::vec::Point;

pub struct Ray {
    origin: Point, 
    direction: Point,
}

impl Ray {
    fn at(self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}