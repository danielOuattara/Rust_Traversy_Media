pub fn run() {
    // immutable variable
    let name = "John Doe";
    println!("{}", name);
    println!("{name}"); // recent syntax

    // mutable variable
    let mut age = 37;
    println!("Age : {}", age);

    age = 38;
    println!("Age : {}", age);

    // constant
    const ID: i32 = 007;
    println!("ID: {}", ID);
    println!("ID: {ID}");

    // Assign multiple variables at once
    let (country, has_children, is_married, number_of_car) = ("France", true, true, 2);

    println!(
        "I live in {}. children with me: {}, Married : {}, Number of car(s) {}",
        country, has_children, is_married, number_of_car
    );
    println!("I live in {country}. children with me: {has_children}, Married : {is_married}, Number of car(s) {number_of_car}"
    );

    println!("-------------------------------------------");
}
