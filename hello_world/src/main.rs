fn main() {
    println!("Hello, world!");
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("Goodbye after 10 sec");
    println!("This is my first program in rust and I am loving it.")
}
