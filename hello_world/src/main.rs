fn main() {
    println!("Hello, world!");
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("Goodbye after 10 sec");
}
