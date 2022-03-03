use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    println!("{:?}", numbers);

    //Re-assign value
    numbers[2] = 20;

    //Add on to vector
    numbers.push(5);
    numbers.push(6);

    //Pop off last value
    numbers.pop();

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

    //Loop through vector
    for i in numbers.iter() {
        println!("Number: {}", i);
    }

    //Loop and mutate values 
    for i in numbers.iter_mut() {
        *i *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}