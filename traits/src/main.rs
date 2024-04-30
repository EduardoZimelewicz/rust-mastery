trait Animal {
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}

impl Animal for Human {
    fn create(name: &'static str) ->  Human {
        Human{name}
    }

    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says hello", self.name);
    }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat{name}
    }

    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says meow", self.name);
    }
}

fn traits() {
    //let h = Human{name: "John"};
    let h = Human::create("John");
    //let c = Cat{name: "Misty"};
    let c = Cat::create("Misty");
    h.talk();
    c.talk();
}

fn main() {
    traits();
}
