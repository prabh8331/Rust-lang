pub fn data_types() {
    // Unsighed integers
    let unsigned_num: u8 = 5; // u16, 132, u64, u128
    println!("num: {unsigned_num}");

    // Signed intergers 
    let signed_num: i8 = 5;
    println!("num: {signed_num}");

    // Floating point numbers
    let float_num: f32 = 5.0;
    println!("float_num: {float_num}");

    // Platform specific integers 
    let arch_1: usize = 5; // usize represents a pointer sized unsigned integer
    let arch_2: isize = 5;  // isizw represients a pointer sized signed integer
    println!("arch: {arch_1}");
    println!("arch: {arch_2}");

    // Characters
    let char: char = 'a';
    println!("char: {char}");

    // Boolean
    let b: bool = true;
    println!("bool: {b}");

    // Type aliasing
    type Age = u8;
    let peter_age: Age = 42; // this is used to enhance the code clearity
    println!("age: {peter_age}");

    // Type Conversion -- can be required for reason such as precision in computation, 
    // or to overcome compatibiility issues between different parts of the code 

    let a=10;
    let b = a as f64; 
    println!("a: {a}");
    println!("b: {b}");
}