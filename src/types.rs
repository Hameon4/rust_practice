// Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (no. of bits they take in memory)
// Floats: f32, f64
//Boolean (bool)
// Characters (char)
// Tuples
// Arrays

pub fn run() {
    // explicit
    let x: i32 = 1;
    let y: f32 = 2.5;

    // implicit
    let z = 3;

    println!("{} {} {}", x, y, z);

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    println!("Max f32: {}", std::f32::MAX);
    println!("Max f64: {}", std::f64::MAX);

    // boolean
    let is_active: bool = true;

    // get boolean from expression
    let is_greater: bool = 10 > 5;

    // char
    let a1: char = 'a';
    let face: char = '\u{1F923}';
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}