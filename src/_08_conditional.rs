pub fn run() {
    let age: u8 = 18;
    let check_id = false;
    let has_parents_auth = true;

    // if/else
    if age >= 21 && check_id {
        println!("you can drive");
    } else {
        println!("I need to check your ID")
    }

    //----------------------------
    // if/else if/else
    if age >= 21 && check_id {
        println!("you can drive");
    } else if age < 21 && check_id {
        println!("too young to drive!")
    } else {
        println!("I need to check your ID")
    }

    //----------------------------
    // if/else if/else with ||
    if age >= 21 && check_id {
        println!("you can drive");
    } else if age < 21 && check_id {
        println!("too young to drive!")
    } else if age < 21 && check_id || has_parents_auth {
        println!("young can drive, but be careful!")
    } else {
        println!("I need to check your ID")
    }

    // short hand if

    let is_of_age = if age > 21 { true } else { false };
    println!("Are you an adult ? {}", is_of_age);
}
