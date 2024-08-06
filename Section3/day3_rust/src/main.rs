//  ----------------------------
//          - Functions
//           fn function_name() {}
//  naming convension of fuction is snake_case
//          - Code Blocks
//  ----------------------------

mod conditionals;
// main funciton which executes first
fn main() {

    my_fn("this is my function");
    let para1 = "pass the variable as a parameter in funciton"; // variable type and paramater type must match
    my_fn(para1);


    // calling the funciton and storing the retrun value of the funciton to the variable 
    let answer = multiplication(10, 20);
    println!("{answer}");

    let result = basic_math(10, 20);
    println!("{:?}",result);

    // destructureing the tuple into individual variabls 
    let (multiplication, addition, subtraction) = basic_math(10, 20);
    println!(" {multiplication} add: {addition} sub: {subtraction}");


    // code blocks - are statements enclosed in curly braces // code blocks can also have return values 
    // whole code block is an assignment statment, so add semicolon in end
    let full_name = {
        let first_name = "Prabh";
        let last_name = "Singh";
        format!("{first_name} {last_name}") // format macro is used for string formatting in rust
    };
    println!("{full_name}");
    // similarities of code blocks and funcitons 
    // 1. they have own separate body, can return values, and may have variables which are limited in scope to their body
    // difference of code blocks and funtions
    // 1. code blocks are not designed for reuse, while funtions are
    // 2. code blocks don't have any explicit parameters
    // 3. all variables in the scope in which the code block lies are visible to it
    // funciton have an explicit list of parameters and can oly have access to varialbles, which are either provied as parameter or are locally defined inside the funciton



    // covering the conditionals topics
    conditionals::conditional();


}

// parameter in fucntion
fn my_fn(parameter1: &str){
    println!("{parameter1}");

}

//value returning function 
// fn multiplication(nam1: i32, num2: i32) -> i32 {}  // here error is intresing by default funciton return unit type and i32 was expected 
fn multiplication(num1: i32, num2: i32) -> i32 {

    println!("COmputing multiplcation"); // as this do not return any value this is statment
    num1*num2   // last code line in the funtion is an expression, since it will return a value for a function
    // also do not write ; if we plan to return that value becaue if we add ; we will again see the type error becaoue then it return unit type
    // this special syntax of not adding ; only works in the last line of the funciotn
} 


// returning multiple values from function using tuple
fn basic_math(num1: i32, num2: i32) -> (i32,i32,i32) {

    (num1 * num2, num1+num2, num1-num2)
}