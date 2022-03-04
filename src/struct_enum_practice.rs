// Struct <start>
struct Student {
    first_name: String,
    last_name: String,
    academic_year: String,
    cgpa: f32 
}

impl Student {
    fn print_info(&self) {
        println!("Full name: {} {} || Academic Year: {} || CGPA: {}", self.first_name, self.last_name, self.academic_year, self.cgpa);
    }
}
// Struct <end>


// Enum <start>
enum Operation {
    Multiply {a: i32, b: i32},
    Divide {a: i32, b: i32},
    Add {a: i32, b: i32},
    Subtract {a: i32, b: i32}
}

fn init_operation(op: Operation) {
    match op {
        Operation::Multiply { a, b } => println!("{}", a * b),
        Operation::Divide { a, b } => println!("{}", a / b),
        Operation::Subtract { a, b } => println!("{}", a - b),
        Operation::Add { a, b } => println!("{}", a + b),
    };
}
// Enum <end>

pub fn run() {
    let stu = Student {
        first_name: "Hamad".to_string(), 
        last_name: "Alsheraifi".to_string(), 
        academic_year: "Junior".to_string(), 
        cgpa: 3.51
    };
    stu.print_info();

    let a: i32 = 8;
    let b: i32 = 4;
    let ops = Operation::Multiply{a, b};
    let ops2 = Operation::Divide {a, b};
    let ops3 = Operation::Add {a, b};
    let ops4 = Operation::Subtract {a, b};

    init_operation(ops);
    init_operation(ops2);
    init_operation(ops3);
    init_operation(ops4);
}