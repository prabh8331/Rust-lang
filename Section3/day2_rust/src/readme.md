# Day2
today coverd variables, constants, data_types and compound_data types

## Practice on Variables
```rs
// Problem 1: Correct the code below.

 fn main(){
    my_age  = 40;
    //let my_age  = 40; ans
    println!("My age is: {}", my_age);  // do not change this line
 }
```

```rs
//Problem 2: Correct the code below

 fn main(){
    let x1 = 40;
    let x2 = x1;
    // let mut x2 = x1; // ans 
    x2 = x1-2;    // do not change this
    println!("x1 is: {} and x2 is: {}", x1,x2); // do not change this
 }
```

```rs
// Problem 3: Without executing. answer if it will comiple? 

fn main() {
    let mut x1 = 40;
    let x2;
    x1 = x1 * 3;
    x2 = x1 - 2;
    println!("x1 is: {}, x2 is: {}", x1, x2);
}
// Explaination: 
// Yes, it will compile. Although the variable x2 is not mutable and we are trying to assign 
// a value to this variable in the line "x2 = x1 - 2" but it is ok since the immutable 
// variables can be assigned values once. 
// Since we are not updating its value later on therefore the variable x2 is assigned 
// a value once which is consistent with the definition of immutable variables, 
// i.e., they can only be assigned value once.
```

```rs
// Problem 4: Fix the code below

fn main() {
    let a = "three";  // don't change this line
    a = 10; // don't change the name of this variable
    // let a = 10; // ans - this is problem of shadowing a = 'three' is shadowed by a=10 
    println!("a is: {}", a); 
}
```


## Practice on Data Types

```rs
// Problem 1: Fix the code below by assigning correct value to the variable


fn main() {
    let x: u8; // Don't change this line!
    x = -1;
    // x = 1; // as x is u8 i.e. unsign variable so removed the - sign
    println!("x is: {}", x);
}
```


```rs
// Problem 2: Make this program compile by replacing the variable type.

fn main() {
    let pi: i32;
    // let pi: f32;  // as pi defined below is float so changing the datatype
    pi = 3.14159; // This value represents pi 
    println!("pi is: {}", pi);
}
```


```rs
// Problem 3: Replace the placeholder "DATA_TYPES_PLEASE" with the appropriate data types in the given program
 
fn main() {
        let a: DATA_TYPES_PLEASE = -15;
        // let a: i32 = -15;
        let b: DATA_TYPES_PLEASE = 170;
        // let b: i32 = 170;
        let name: DATA_TYPES_PLEASE = "Michael";
        // let name: &str = "Michael";
        println!("name is: {}, and the multiplication result is {}", name, a * b);

}
```

```rs
// Problem 4: Add a type alias for Book so that we are able to store the information.

fn main() {
    type Book = (String, String, u32);// Add your code here to this line
    // type Book = (String, String, u32);   // ans in () define the type of values

    let book1: Book = (
        String::from("Rust Programming Langauge"),
        String::from("RUST Community"),
        2010,
    );
    println!(
        "Book name: {}, Author: {}, Year {}",
        book1.0, book1.1, book1.2
    );

    let book2: Book = (
        String::from("Rust by Example"),
        String::from("Steve Klabnik and Carol Nichols"),
        2015,
    );
    println!(
        "Book name: {}, Authors: {}, Year {}",
        book2.0, book2.1, book2.2
    );
}

```


