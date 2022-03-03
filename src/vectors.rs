use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    println!("{:?}", numbers);

    //Re-assign value
    numbers[2] = 20;

    // get vector length 
    println!("Length of vector: {}", numbers.len());

    println!("{:?}", numbers);

    // get single values 
    println!("single value: {}", numbers[0]);

    // vectors are stack allocated - get size in bytes
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2]; 
    println!("Slice: {:?}", slice);
}