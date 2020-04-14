
#[derive(Debug)]
pub struct Point {
    pub x : f64,
    pub y : f64,
}

impl Point {
    pub const ORIGIN : Point = Point{x:0., y:0.};

    pub fn new(x : f64, y : f64) -> Point {
        return Point{x, y};
    }

    pub fn add(&self, other : Point) -> Point {
        return Point::new(self.x + other.x, self.y + other.y);
    }

    pub fn dist_(a : &Point, b : &Point) -> f64 {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        return (dx * dx + dy * dy).sqrt();
    }

    pub fn dist(&self, b : &Point) -> f64 {
        return Point::dist_(self, b);
    }
}
