fn vector() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);

    for x in &a { println!("{}", x) }

    let last_elem = a.pop();
    println!("last elem = {:?}, a = {:?}", last_elem, a);

    while let Some(x) = a.pop() {
        println!("{}", x)
    }
}

fn main() {
    vector();
}
