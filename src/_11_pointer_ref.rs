// Reference Pointers -> Points to a resource in memory

pub fn run() {
    // primitive array

    let array_1 = [1, 2, 3];
    let array_2 = array_1;

    println!("array_1 = {:?}", array_1);
    println!("array_2 = {:?}", array_2);

    /*
    With non-primitives, if you assign another variable to a piece
    of data, the first variable will no longer hold that value.
    You will need to use a references (&) to point to the resource */

    let vec_1 = vec![1, 2, 3];
    // let vec_2 = vec_1; // Incorrect according to above statement
    let vec_2 = &vec_1; // solution

    println!("vec_1 = {:?}", vec_1);
    println!("vec_2 = {:?}", vec_2);
}
