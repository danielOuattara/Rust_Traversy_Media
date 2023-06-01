pub fn run() {
    // print to console
    println!("Hello from _01_print");

    // basic formatting
    println!("Number is : {}", 1);
    println!("{} {} is from {}", "John", "Doe", "no where");

    // Positional Arguments
    println!("{2} {1} is from {0}", "no where", "Doe", "John");

    // Named Arguments
    println!(
        "{first_name} {last_name} is from {place}",
        place = "no where",
        last_name = "Doe",
        first_name = "John"
    );

    // Placeholder traits

    println!("Binary: {:b}", 23);
    println!("Octal: {:o}", 23);
    println!("Hexadecimal: {:x}", 23);

    // placeholder debug trait
    println!("country: {:?}", "Afghanistan");

    // Basic Math : todo, to easy, forget
    println!("-------------------------------------------");
}
