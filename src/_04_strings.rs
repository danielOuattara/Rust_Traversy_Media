// Primitive string: immutable, fixed-length, some where in the memory

// String: growable, heap-allocated data structure, used when needed to modify
pub fn run() {
    let primitive_string = "Hello world";
    println!("{}", primitive_string);

    let mut growable_string = String::from("Hello world");
    println!("{}", growable_string);

    // length
    println!("length = {}", growable_string.len());

    // grow string
    growable_string.push('!');
    println!("{}", growable_string);

    growable_string.push_str(" of Rust !");
    println!("{}", growable_string);

    // string capacity in bytes
    println!(
        "capacity of growable_string: {}",
        growable_string.capacity()
    );

    // check if empty
    println!(
        "growable_string is empty ? : {}",
        growable_string.is_empty()
    );

    // contains some sub string
    println!(
        "growable_string contains the word \"world\" : {}",
        growable_string.contains("world")
    );

    // replace sub string
    println!("{}", growable_string.replace("world!", "every body"));

    // loop through string by whitespace
    for word in growable_string.split(" ") {
        println!("{}", word)
    }

    for word in growable_string.split_whitespace() {
        println!("{}", word)
    }
}
