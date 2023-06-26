//
// Arrays : fixed list where elements are of the same data types

use std::mem;

pub fn run() {
    //declare an array

    let array_1 = [1, 2, 3, 4, 5]; // auto inference
    let array_2: [i32; 4] = [6, 7, 8, 9]; // manual inference

    println!("{:?}", array_1);
    println!("{:?}", array_2);

    let heros_name = ["John Snow", "Deanearys Targarean", "Rod Stewart"];
    println!("{:?}", heros_name);

    // get single value

    println!("First hero is {:?}", heros_name[0]);
    println!("First hero is {}", heros_name[0]); // see the difference !

    // re-assign

    let mut pets_name = ["kriko", "georges", "doudou"];
    println!("{:?}", pets_name);

    pets_name[2] = "kiss-kiss";
    println!("{:?}", pets_name);

    // get array length

    println!("{}", pets_name.len());

    // Array are stack allocated

    println!("'array_1' occupies {} bytes.", mem::size_of_val(&array_1));
    println!("'array_2' occupies {} bytes.", mem::size_of_val(&array_2));
    println!(
        "`heros_name occupies` {} bytes.",
        mem::size_of_val(&heros_name)
    );
    println!(
        "'pets_name' occupies {} bytes.",
        mem::size_of_val(&pets_name)
    );

    //get slices

    let slice_1 = &array_1;
    println!("slice_1 = {:?} ", slice_1);

    let _slice_2: &[i32; 4] = &array_2;

    let slice_1_bis = &array_1[0..2];
    println!("slice_1_bis = {:?} ", slice_1_bis);
}
