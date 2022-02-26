pub fn run() {
    // print to console
    println!("Hello from the print.rs file!");

    // basic formatting
    println!("{} plus {} is: {}", "5", "2", 5 + 2);

    // positional argument
    println!("{0} is from {1} and {0} likes to {2}", "Hamad", "UAE", "code");

    // Name argument
    println!("{name} likes to play {activity}", name = "Hamad", activity = "Table Tennis");

    // placeholder traits (bin, oct, hex formats)
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}