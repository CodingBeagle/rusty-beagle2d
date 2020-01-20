fn main() {
    let mut hello_world = String::from("Hello, World! :D");
    println!("My string is: {}", hello_world);

    hello_world.push_str(" Extra Text!");
    println!("Now the string is: {}", hello_world);
}