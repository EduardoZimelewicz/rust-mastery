fn main() {
    let vec = vec![3,2,1];

    //for x in &vec {
      //  println!("{}", *x);
    //}

    for x in vec.iter() {
        println!("{}", x);
    }

    for x in vec.iter().rev() {
        println!("{}", x);
    }
}
