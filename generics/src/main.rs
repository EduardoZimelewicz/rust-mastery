struct Point<T, V> {
    x: T,
    y: V
}

fn generics() {
    let a:Point<u16, i32> = Point { x: 0, y: 4 };
    let b:Point<f64, f64> = Point { x: 1.2, y: 3.4 };
}

fn main() {
    generics();
}
