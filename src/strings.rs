// Primitive str = immutable fixed-length in memory
// (String) growable = heap-allocated data structure - use when needed to modify string data

pub fn run() {
    // growable example using the String keyword. erase keyword if want primitive
    let mut hello = String::from("Hello ");

    // get length
    println!("Length: {}", hello.len());

    // Push Char
    hello.push('W');

    // Push String
    hello.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check string if empty
    println!("isEmpty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World","There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }


    // Create string with capacity 
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{} - Size: {} - Capacity: {}", s, s.len(), s.capacity());

    // Assertion testing 
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", hello);
}