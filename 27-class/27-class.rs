struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn greet(&self) {
        println!("Hello, {} {}", self.first_name, self.last_name);
    }
}

fn main() {
    let p1 = Person {
        first_name: "Peter".to_string(),
        last_name: "Wang".to_string(),
    };
    p1.greet();
    let p2 = Person {
        first_name: "William".to_string(),
        last_name: "Zhang".to_string(),
    };
    p2.greet();
}
