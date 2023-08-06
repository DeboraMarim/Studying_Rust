struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    let person2 = Person {
        name: String::from("Bob"),
        age: 25,
    };

    println!("Nome: {}, Idade: {}", person1.name, person1.age);
    println!("Nome: {}, Idade: {}", person2.name, person2.age);
}