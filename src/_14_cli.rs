use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();

    println!("Args: {:?}", args);
    println!("Commands: {}", command);

    let name = "John";
    let status = "100%";

    if command == "hello" {
        println!("Hello {name} ! How are you ?");
    } else if command == "status" {
        println!(" {status} ");
    }
}
