//Reference Pointers  = Point to a resource in memory

pub fn run() {
    // Primitive array
    let arr1 = [1, 2, 3];
    let _arr2 = arr1;

    //Non-primitive
    // With non-primitives, if you assign another variable to a piece of data, 
    //the first vairable will no longer holds that value. You'll need to use 
    //a reference (&) to point to the resource
    
    // Vector
    let v1 = vec![1, 2, 3];
    let v2 = &v1;

    println!("Values: {:?}", (&v1, v2));
}