struct Creature {
    name: String
}

impl Creature {
    fn name(name: &str) -> Creature {
        println!("{} enters the game", name);
        Creature {name: name.into()}
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} is dead", self.name)
    }
}

fn main() {
    let goblin = Creature::name("Jeff");
    println!("game proceeds");
    drop(goblin);
}