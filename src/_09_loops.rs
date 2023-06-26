pub fn run() {
    let mut count = 0;

    // Infinite loop
    loop {
        println!("{count}");
        if count == 10 {
            break;
        }
        count += 1;
    }

    println!("------------------");

    // while loop
    let mut i = 0;
    while i != 5 {
        println!("{:?}", i);
        i += 1;
    }

    // for loop

    for item in 0..10 {
        println!("{item}");
    }
}
