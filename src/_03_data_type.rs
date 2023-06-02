pub fn run() {
    // primitive types
    /*
    integers
    floats
    boolean
    characters
    Tuples
    Arrays
    */

    // Default integer is i32
    let _x = 1;

    // Default float is i64
    let _y = 3.14;

    // add explicit type
    let _y: f64 = 3.14;

    // find max size

    let _x: i32 = i32::MAX;
    // OR
    println!("Max for i32 : {}", std::i32::MAX);
    println!("Max for i64 : {}", std::i64::MAX);

    // booleans
    let _is_active = true;
    let _is_pilot: bool = false;

    let _is_greater_than_5 = 10 > 5;

    // char
    let _a = 'a';
    let _single_char = 'z';
    let _face = '\u{1F600}';
    println!("{}", _face);

    println!("-------------------------------------------");
}
