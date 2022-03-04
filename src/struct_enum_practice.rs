struct Student {
    first_name: String,
    last_name: String,
    academic_year: String,
    cgpa: f32 
}

enum operation {
    Multiply {a: i32, b: i32},
    Divide {a: i32, b: i32},
    Add {a: i32, b: i32},
    Subtract {a: i32, b: i32}
}

fn init_operation(op: operation) {
    match op {
        operation::Multiply { a, b } => println!("{}", a * b),
        operation::Divide { a, b } => println!("{}", a / b),
        operation::Subtract { a, b } => println!("{}", a - b),
        operation::Add { a, b } => println!("{}", a + b),
    };
}

pub fn run() {
    let stu = Student {
        first_name: "Hamad".to_string(), 
        last_name: "Alsheraifi".to_string(), 
        academic_year: "Junior".to_string(), 
        cgpa: 3.51
    };
    println!("Full name: {} {}\nAcademic Year: {}\nCGPA: {}", stu.first_name, stu.last_name, stu.academic_year, stu.cgpa);

    let a: i32 = 8;
    let b: i32 = 4;
    let ops = operation::Multiply{a, b};
    let ops2 = operation::Divide {a, b};
    let ops3 = operation::Add {a, b};
    let ops4 = operation::Subtract {a, b};

    init_operation(ops);
    init_operation(ops2);
    init_operation(ops3);
    init_operation(ops4);
}