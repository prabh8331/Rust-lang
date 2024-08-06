# Day3

## Practice on Funcitons

```rs
// Problem 1: Add the missing functions to the code 

// add_3(x): This function should add three to the input variable ‘x’ and the return the resultant value.
// add_5(x): This function should add five to the input variable ‘x’ and the return the resultant value.
// times(x,y): This function should compute the multiplication of the inputs ‘x’ and ‘y’ and return the resultant value.

fn main() {
    let x = 3;
    let y = 4;
    println!(
        "The result of x+3 times y+5 is {}",
        times(add_3(x), add_5(y))
    );
}

// ans bellow
fn add_3(x:i32) -> i32 {
    x+3
}


fn add_5(y:i32) -> i32 {
    y+5
}

fn times(x:i32, y:i32) -> i32 {
    x*y
}

```

```rs

// Problem 2: Refactor the code by taking rid of the variables x and y.
/* Rewrite the code in a way that produce the  same outcome as the original code, but without using any variables */ 


fn double(x: i32) -> i32 {
    x * 2
}

fn triple(x: i32) -> i32 {
    x * 3
}

fn main() {
    let x = triple(double(5));
    let y = triple(x);
    println!("Answer: {}", y);
    // ans below and above two gets removed
    println!("Answer: {}", triple(triple(double(5))));
}

```


```rs
// Problem 3: Correct the code below so that it compiles 

fn print_distance(point: (f32, f32)) -> f32 {
    let (x, y) = point;
    (x.powf(2.0) + y.powf(2.0)).sqrt() // Formula for computing distance
}

fn main() {
    println!(
        "The distance of the number the point from the origin is {}",
        print_distance(5.0, 4.0) // concentrate on the call to the function 
        // print_distance((5.0, 4.0)) // ans
    );
}

```


```rs
//Problem 4: Add the definition of the quadruple function below by calling the double function twice inside the quadruple function.   

fn double(x: i32) -> i32 {
    x * 2
}

fn quadruple(x: i32) -> i32 {
    // your code here //  
    double(double(x))
}

fn main() {
    println!(
        "For 1: the expected value is 4 while the output is {}",
        quadruple(1)
    );

    println!(
        "For 2: the expected value is 8 while the output is {}",
        quadruple(2)
    );

    println!(
        "For 3: the expected value is 12 while the output is {}",
        quadruple(3)
    );

    println!(
        "For 4: the expected value is 16 while the output is {}",
        quadruple(4)
    );
}

```
