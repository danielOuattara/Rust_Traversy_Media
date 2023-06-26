fn greetings(greet: &str, name: &str, action: &str) {
    println!("{greet} {name}, nice to {action} you !");
}

fn add_number_1(number1: i32, number2: i32) -> i32 {
    number1 + number2
}

fn add_number_2(number1: i32, number2: i32) -> i32 {
    return number1 + number2;
}

pub fn run() {
    greetings("Hello", "John", "meet");
    greetings("Welcome", "Bob", "see");

    // bind function to variables

    let get_sum_1 = add_number_1(5, 8);
    println!("{get_sum_1}");

    let get_sum_2 = add_number_2(15, 28);
    println!("{get_sum_2}");

    // closures
    let add_numbers = |n1: i32, n2: i32| n1 + n2;
    println!("Closure 1 sum {}", add_numbers(3, 4));

    let n3: i32 = 10;
    let add_numbers = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure 2 sum {}", add_numbers(3, 4));
}
