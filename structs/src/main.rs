struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

fn structures(){
    let p = Point { x: 3.0, y: 4.0 };
    println!("point p is at ({}, {})", p.x, p.y);

    let p2 = Point { x: 5.0, y: 10.0 };
    println!("point p2 is at ({}, {})", p2.x, p2.y);

    let my_line = Line { start: p, end: p2 };
    println!("line starts at ({},{}) and ends at ({},{})",
             my_line.start.x, my_line.start.y, my_line.end.x, my_line.end.y)
}

fn main() {
    structures();
}
