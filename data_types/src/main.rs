#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

fn main() {
    let person = Person {
        name: String::from("satyam raj"),
        age: 25,
    };

    println!("{:?}", person);
}
