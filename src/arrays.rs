// Arrays - fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    //Re-assign value
    numbers[2] = 20;

    // get array length 
    println!("Length of array: {}", numbers.len());

    println!("{:?}", numbers);

    // get single values 
    println!("single value: {}", numbers[0]);

    // arrays are stack allocated - get size in bytes
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2]; 
    println!("Slice: {:?}", slice);
}