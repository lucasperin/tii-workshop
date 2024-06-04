
pub trait ShapeArea {
    fn area(&self) -> Option<f32>;
}

pub trait ShapePerimeter {
    fn perimeter(&self) -> Option<f32>;
}

pub enum Shape {
    Point(Point),
    Circle(Circle),
    Triangle(Triangle),
    Rectangle(Rectangle)
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl ShapeArea for Point {
    fn area(&self) -> Option<f32> {
        None
    }
}

impl ShapePerimeter for Point {
    fn perimeter(&self) -> Option<f32> {
        None
    }
}

pub struct Circle {
    pub radius: u32,
}

impl ShapeArea for Circle {
    fn area(&self) -> Option<f32> {
        Some((self.radius * self.radius) as f32 * std::f32::consts::PI)
    }
}

impl ShapePerimeter for Circle {
    fn perimeter(&self) -> Option<f32> {
        Some((2u32 * self.radius) as f32 * std::f32::consts::PI)
    }
}

pub struct Rectangle {
    pub a: Point,
    pub b: Point,
}

impl ShapeArea for Rectangle {
    fn area(&self) -> Option<f32> {
        let base = (self.a.x - self.b.x).abs();
        let height = (self.a.y - self.b.y).abs();
        Some((base * height) as f32)
    }
}

impl ShapePerimeter for Rectangle {
    fn perimeter(&self) -> Option<f32> {
        let base = (self.a.x - self.b.x).abs();
        let height = (self.a.y - self.b.y).abs();
        Some((2 *( base + height)) as f32)
    }
}

pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_area() {
        let p = Point {
            x: 1,
            y: 2,
        };
        assert!(p.area().is_none());
    }

    #[test]
    fn point_perimeter() {
        let p = Point {
            x: 1,
            y: 2,
        };
        assert!(p.perimeter().is_none());
    }
}
