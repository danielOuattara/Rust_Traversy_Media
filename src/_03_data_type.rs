pub fn run() {
    // primitive types
    /*
    Integers: u8, u16, u32, u64, u128, i8, i16, i32, i64, i128
    Floats:f32, f64
    Boolean (bool)
    Characters (char)
    Tuples
    Arrays (fixed length)
    Vectors (growable)
    */

    // Default integer is i32
    let _x = 1;

    // Default float is i64
    let _y = 3.14;

    // add explicit type
    let _y: f64 = 3.14;

    // find max size
    let _x: i32 = i32::MAX;
    println!("i32::MAX = {_x}");

    // OR

    println!("Max for i32 : {}", std::i32::MAX);
    println!("Max for i64 : {}", std::i64::MAX);

    // booleans
    let _is_active = true; // automatic inference
    let _is_pilot: bool = false; // manual inference

    let _is_greater_than_5 = 10 > 5;

    // char
    let _a = 'a';
    let _single_char = 'z';
    let _face = '\u{1F600}';
    println!("{}", _face);

    println!("-------------------------------------------");
}
