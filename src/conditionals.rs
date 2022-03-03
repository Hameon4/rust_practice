pub fn run() {
    let age = 18;

    if age >= 21 {
        println!("Bartender: What drink?");
    }
    else {
        println!("Bartender: Sorry...");
    }

    //shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("If of age: {}", is_of_age);
}