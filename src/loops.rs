pub fn run() {
    //let mut count = 0; 

    //infinite loop
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);

    //     if count == 20 { 
    //         break;
    //     }
    // }

    let mut fizz_buzz: Vec<u8> = Vec::new();
    let mut fizz: Vec<u8> = Vec::new();
    let mut buzz: Vec<u8> = Vec::new();
    // While loop (FizzBuzz)
    // while count <= 100 {
    //     if count % 15 == 0 {
    //         // println!("FizzBuzz");
    //         fizz_buzz.push(count);
    //     }
    //     else if count % 3 == 0 {
    //         // println!("Fizz");
    //         fizz.push(count);
    //     }
    //     else if count % 5 == 0 {
    //         // println!("Buzz");
    //         buzz.push(count);
    //     }
    //     else {
    //         println!("{}", count);
    //     }
    //     count += 1;
    // }
    
    // for range 
    for i in 0..100 {
        if i % 15 == 0 {
            fizz_buzz.push(i);
        }
        else if i % 3 == 0 {
            // println!("Fizz");
            fizz.push(i);
        }
        else if i % 5 == 0 {
            // println!("Buzz");
            buzz.push(i);
        }
        else {
            println!("{}", i);
        }
    }
    println!("FizzBuzz: {:?}\nFizz: {:?}\nBuzz: {:?}", fizz_buzz, fizz, buzz);

}