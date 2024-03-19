#![allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue
}

fn enums() {
    let c:Color = Color::Red;

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
    }
}

fn main() {
    enums();
}