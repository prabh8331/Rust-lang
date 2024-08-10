// -------------------------------------------------
// 			Bonus
// 			- Comments
// 			- More on Printing
//			- Inputs
//			- Variable Convention
//			- Statics
// -------------------------------------------------
pub fn basics_info() {
    // The current line is a comment line
    // This is the second line of comment

    /* This is a
    multiple line
    comment
    */

    print!("This is a print command");
    print!("This is going to be printed on the same line");

    /* Escape sequences
    \n : Newline character.
    \t : Tab space.
    \r : Carriage Return.
    \" : Double quote.
    \\ : Backward slash.
    */

    println!("\n Will be printed after one empty line");
    println!("\t A tab space at the start");
    println!("This will be overwritten \r This text will only appear on the screen");

    println!("Prints double quotes \", Prints backslash \\");

    // some basics of print - 
    println!(
        "I am doing {2} from {1} years and i {0} it",
        "like", 20, "programming"
    );

    println!(
        "{language} is a system programming language which is cool to {activity} in.",
        activity = "code",
        language = "Rust"
    );


    // rust uses the standard input output library to get input from the user
    // specifically read line funciton is used 

    // define the sting variable
    let mut n = String::new();

    // pass n to read_line funtion  std::io::stdin().read_line(&mut n)    .expect is given in case of input read fail
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");

    let n: f64 = n.trim().parse().expect("invalid input");       // expect if input can be passed into requested type
    println!("{n}");


    // varialbe convension and unused variable - in rust variable naming convension is snake_case
    // by adding _ in start of rh variable can ignore the unused variable warining
    let _number_one = 45.0;

    // can use _ in numbers to improve the readblility of large numbers
    let x = 40_000;


    //  static varialble (similar to constants) like constant need to provide explict type annotation, naming convension is SCREAMING_SNAKE_CASE
    static WELCOME: &str = "Welcome to Rust"; //static
    const PI: f32 = 3.14; // constant 


    // Difference b/w the static and constant is that constants are inlined -- 

    // inlining of constants refers to the fact that they are replaced with their concrete values
    // in following case the value of pi .. this means they do not occupy a specifc locaiotn in memory
    let a = PI;
    let b = PI;


    // but statics do occupy specific memory locaton, i.e. there is one instance of the value 
    // all reference to the static refer to the same memory location 
    let c = WELCOME;
    let d = WELCOME;
    // Reasons for using the statics include.When you want to refer to some larger amount of data in memory, or when you are interested in interior mutability.

}


// constants can be define in Gloabal/local/inside a funtion scope
// rust is statically typed language not dynamically typed - Rust is statically-typed as the type of a variable is known at compile-time instead of at run-time


// Consider the following program given below. Guess the output of this program.
// Note: the range for u32 is from 0 ... 4294967295
// fn main() {
//     let x: u64 = 4_294_967_296;
//     let y = x as u32;
//     if x == y as u64 {
//         println!("x equals y.");
//     } else {
//         println!("x does not equal y.");
//     }
// }\

// output - x does not equal y 
// reason - The 'as' keyword in Rust performs type casting, but it is lossy, meaning that unnecessary information can be discarded. When we use 'as' to convert between types, there is a risk of losing data without any warning. In the given example, the variable 'y' is assigned the value 4,294,967,296, which exceeds the maximum value that can be stored in a u32 type. As a result, the value is truncated. It is surprising that neither the Rust compiler, nor the Clippy, or the runtime generates any warning or error to indicate that data loss has occurred.



// What should we put in place of the ? in the code segment below so that the code compiles
// fn main() {
//     let mut shopping_list: Vec<?> = Vec::new();
//     shopping_list.push("milk");
// }
// ans - &str