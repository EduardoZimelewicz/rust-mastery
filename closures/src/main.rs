fn say_hello() {
    println!("hello");
}

fn closures() {
    let sh = say_hello;
    sh();

    let plus_one = |x:i32| -> i32 {x + 1};

    let a = 6;
    println!("{}", plus_one(a));


}

fn main() {
    closures();
}
