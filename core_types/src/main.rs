#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;
use std::mem::size_of_val;

fn main() {
    let a: u8 = 123;
    println!("a = {}", a);

    let mut b: i8 = 0;
    println!("b = {} before", b);
    b = 42;
    println!("b = {} after", b);

    let mut c = 123456789;
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}", c);

    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z*8);

    let d: char = 'x';
    println!("{} is a char, size = {} bytes", d, size_of_val(&d));

    let e: f32 = 2.5;
    println!("{}, size = {} bytes", e, size_of_val(&e));

    let g: bool = false;
    println!("{}, size = {} bytes", g, size_of_val(&g));
}