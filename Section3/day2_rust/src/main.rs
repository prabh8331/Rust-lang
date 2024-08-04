mod data_type;
mod compound_data_type;


fn main() {
    // Variables
    // Define the variable, based on the variable value rust will annotate type
    let a = 10; //rust auto detect the data type
    let b: i16 = 10; // have given manually 
    let c=10.0;
    // let d: i16=10.0; // as this is defied as interger and values is inserted as float will give compile error
    println!(" value of a:{a} , b: {b} c: {c} ");

    // Mutability - In rust variables are inmutabile by default
    let x = 5;
    // x=10;  this will not work as x is not spacified it is mutable

    let mut y = 120;
    y = 19;
    println!("value of x {x}, y {y}");


    // Scope - each variable in rust lives within the certain scope - 
    // scope means a code line rsides b/w opening and closing braces
    {
        let z = 50;
    }
    // println!("value of z: {z}");  // this will give error thact cannot find z in this scope

    // Shadowing
    // recreat a same variable this this bint t  to the and 2nd variable is what compiler will see 
    // this is called 2nd variable shadows the first variable
    let t = 10;
    let t = t+t;
    println!("t:{t}");
    // difference b/w the mutability and shadowing is that the mutability allows you to alter a single value
    // but shadowing involves creatin of two distinct variables, with one of them shadowing the other
    // shadowing can be useful where initially wanted varaiable to be of some type but after some operation you want to 
    // change the to anoter type 
    let u = 3;
    let u = 3.2;
    println!("u: {u}");

    let v = 30;
    {
        let v = 40;
        println!("inner v is : {v}");
        // here innver v is shadowing the outer v , so this is handy in situations where the same variable needs to be reused in the inner scope, possible in different context
    }
    println!("v is : {v}");

    // Constants - declared using the const keyword and required explict type annotation, 
    // once created constant can't be mutated, constant must be in screamming   SNAME_CASE  
    const MAX_VALUE : i32 = 100; 
    
    println!("max_value is : {MAX_VALUE}");

    


    // this code will execute as we are not assigining x2 whle defining but assiging it only once
    fn main() {
    let mut x1 = 40;
    let x2;
    x1 = x1 * 3;
    x2 = x1 - 2;
    println!("x1 is: {}, x2 is: {}", x1, x2);
    }


    // This section covers the data types which holds sigle values 
    data_type::data_types();

    // This section covers the compound data_types which can hold multiple data types 
    compound_data_type::compound_data_types();

}
