struct Point<T> {
    x: T,
    y: T,
}

struct Line<T> {
    start: Point<T>,
    end: Point<T>,
}

pub(crate) fn generics_demo() {
    let a = Point { x: 0.0, y: 0.0 };
    let b = Point { x: 1.2, y: 3.4 };

    // won't work initially
    //let myline = Line { start: a, end: b };
}
