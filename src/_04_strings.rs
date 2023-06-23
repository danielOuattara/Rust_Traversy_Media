// Primitive string --> str: immutable, fixed-length, somewhere in the memory

// String --> growable, heap-allocated data structure, used when needed to modify

pub fn run() {
    let primitive_string = "Hello world";
    println!("{}", primitive_string);

    let mut growable_string = String::from("Hello world");
    println!("{growable_string}");

    // length
    println!("primitive string length = {}", primitive_string.len());
    println!("String length = {}", growable_string.len());

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

    // create string with capacity
    let mut new_string = String::with_capacity(10);
    println!("new_string capacity = {}", new_string.capacity());
    new_string.push('a');
    new_string.push('b');

    // Assertion testing: throw panic if assertion test fails
    assert_eq!(2, new_string.len()); // OK
                                     // assert_eq!(3, new_string.len()); // Not OK: throw a panic
    assert_eq!(10, new_string.capacity()); // OK

    println!("{}", new_string);
}
