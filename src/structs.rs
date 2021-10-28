// structures!
struct A; // does absolutely nothing

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x; // Ctrl+D
        let dy = self.start.y - self.end.y;
        return (dx * dx + dy * dy).sqrt();
    }
}

pub(crate) fn struct_demo() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("point p is at ({},{})", p.x, p.y);

    let p2 = Point { x: 5.0, y: 10.0 };
    let myline = Line { start: p, end: p2 };

    // destructuring

    // member functions
    println!("line length is {}", myline.len());
}
