fn main() {
    println!("Hello, world!");
    another_function();
    int_function(100);
}

fn another_function(){
    println!("hello from another function");
}

fn int_function(num:i32){
    println!("this is the number {}", num);
}