
const MEANING_OF_FILE:u8 = 42;

static mut Z:i32 = 123;

fn main() {
    println!("{}", MEANING_OF_FILE);
    unsafe {
        Z = 777;
        println!("{}", Z);
    }
}