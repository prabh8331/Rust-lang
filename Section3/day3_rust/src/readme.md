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

## Practice on Conditionls and Control Flow

```rs
// Problem 1:
/*
Write a program to find the difference between the square of the sum and the sum of the squares of the first N numbers.
N will be a user-defined input that your program will take.

For example, if the user enters the number 5, you should compute the square of the sum as (1 + 2 + 3 + 4 + 5)^2 = 225.
Next, compute the sum of the squares as (1^2 + 2^2 + 3^2 + 4^2 + 5^2) = (1 + 4 + 9 + 16 + 25) = 55.
Finally, calculate the difference as 225 - 55 = 170.
*/

fn main() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: i32 = n.trim().parse().expect("invalid input");

    let mut square_of_sum = 0;
    let mut sum_of_squares = 0;
    
    /* Complete the code after this line */
    for i in 1..=n {
        square_of_sum=square_of_sum+i;
        sum_of_squares=sum_of_squares+i.pow(2);
    }
    println!("result: {}",square_of_sum.pow(2)-sum_of_squares);

}


```

```rs
// Problem 2:

/*
Write a program to find the sum of natural numbers below a given number N, where N is provided by the user.
The sum should only include numbers that are multiples of either 3 or 5.
For example, if the user enters N = 20, the multiples of 3 are 3, 6, 9, 12, 15, 18, and the multiples of 5 are 5, 10, and 15.

Please note that the value of 15 will be considered only once since it is a multiple of both 3 and 5.
The sum will be calculated as follows: 3 + 5 + 6 + 9 + 10 + 12 + 15 + 18.

Write a program that takes the user input N, performs the necessary calculations, and outputs the sum.
*/

fn main() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: i32 = n.trim().parse().expect("invalid input"); 
    
    
    /* Add your code below this line */   
    let mut n_sum=0;
    
    for i in 1..=n {
        if i % 3 == 0 || i % 5 ==0 {
            n_sum=n_sum+i;
            // println!("{i}  {n_sum}");
        }
    }
    
    println!("result: {n_sum}");
    
}


// solution 2 \\

    let numbers = 1..n;
    let mut combined_list = vec![0];
    for i in 1..n as usize {
        if i % 3 == 0 || i % 5 == 0 {
            combined_list.push(1)
        } else {
            combined_list.push(0);
        }
    }

    let mut values_of_multiples: Vec<i32> = vec![0];
    let mut summation = 0;
    for i in 1..n {
        values_of_multiples.push(combined_list[i as usize] * i);
        summation += values_of_multiples[i as usize];
    }

    println!("\n\n The sum of the multiples are = {}\n\n", summation);



```

```rs

// Problem 3:

/*
This question involves writing code to analyze the production of an assembly line in a car factory.
The assembly line has different speeds, ranging from 0 (off) to 10 (maximum).
At the lowest speed of 1, the assembly line produces a total of 221 cars per hour.
The production rate increases linearly with the speed,
meaning that a speed of 4 produces 4 * 221 = 884 cars per hour.

However, higher speeds increase the likelihood of producing faulty cars that need to be discarded.
The success rate depends on the speed, as shown in the table below:
· Speeds 1 to 4: 100% success rate.
· Speeds 5 to 8: 90% success rate.
· Speeds 9 and 10: 77% success rate.

You need to write two functions:
1. The first function, total_production(), calculates the total number of cars successfully produced without faults within a specified time given in hours. The function takes the number of hours and speed as input and returns the number of cars successfully produced.
2. The second function, cars_produced_per_minute(), calculates the number of cars successfully produced per minute. The function takes the number of hours and speed as input and returns the number of cars produced per minute.
Write the code for both functions based on the provided specifications.
*/


fn main() {
    println!("{}", total_production(6, 5) as i32); // to round the values we use i32. just ignore for mow
    println!("{}", cars_produced_per_minutes(6, 5) as i32); // to round the values we use i32. just ignore for mow
}

fn total_production(hours: u8, speed: u8) -> f32 {
    let success_rate: f32;
    /* Your code below this line*/
    if speed <= 4 {
        success_rate = 1.0;
    } else if speed <= 8 {
        success_rate = 0.9;
    } else {
        success_rate = 0.77;
    }
    success_rate*speed as f32 *hours as f32*221.0

}

fn cars_produced_per_minutes(hours: u8, speed: u8) -> f32 {
    let success_rate: f32;
    /* Your code below this line*/
    if speed <= 4 {
        success_rate = 1.0;
    } else if speed <= 8 {
        success_rate = 0.9;
    } else {
        success_rate = 0.77;
    }
    (success_rate*speed as f32*221.0)/60.0
    
}

// key point in both total_production and cars_produced_per_minutes the way when multiplicaton is done during the return
// success_rate is f32, speed is convered into f32 and 60.0 (.0 is specified to make is floating point) .. because ingetger can't be multiplied by f32 in rust 


```

```rs
// Problem 4:

/*
A palindrome is a word, verse, or sentence that reads the same backward or forward,
such as 'Able was I ere I saw Elba,' or a number like 1881.

Write a function named is_palindrome() that checks whether a given string is a palindrome or not.
The function should take a string as input and return a boolean value indicating whether the string is a palindrome or not.
*/

fn main() {
    let input = String::from("11211");
    println!(
        "It is {:?} that the given string is palindrome",
        palindrome(input)
    );
}

fn palindrome(input: String) -> bool {
    /* Your Code here */
    
    // create a is_palindrome variable and as its value can be changed it is muitalble 
    let mut is_palindrome = true;
    
    // if empty string the considering it as palindrome
    if input.len() == 0 {
        is_palindrome=true;
    }
    else {
        // pull the first and last index of the sting
        let mut last = input.len() -1;
        let mut first = 0;
        
        // convert sting into - byte slice (&[u8])
        let my_vec = input.as_bytes();
        println!("{:?}",my_vec);
        
        while last>first {
            if my_vec[first] != my_vec[last] {
                is_palindrome=false;
                break;
            }
            
            last = last - 1;
            first = first +1;
        }
    }
    is_palindrome
}


```

```rs 
// Problem 5 is tricky so added in revisit problem
```

```rs
// Problem 6: Write a function that implements the logic,
// 'You can see the movie if you are 17 or older, or if you are 13 or older and have a parent's permission.'
// Note: This means that if you 17 or older, you implicitly have permission. 

fn can_see_movie(age: i32, permission: bool) -> bool {
    // Write your code here to implement the logic
    if age >= 17 {
        return true;
    } else if age >= 13 && permission == true {
        return true;
    }  else {
        return false;
    }
}

// solution2 
fn can_see_movie(age: i32, permission: bool) -> bool {
    (age >= 17) || (age >= 13 && permission)
}

fn main() {
    println!("John who is 18, can see the move: {}", can_see_movie(18, true));
}
```


## full code 

```rs
mod conditionals {
pub fn conditional() {
    // Conditionals
    // if else 
    let num = 40;
    if num < 50 {
        println!("The number is less than 50");
    }
    else {
        println!("The number is greater than or equal to 50");
    }
    

    let marks=65;
    // let mut grade = 'N';

    // if marks >= 90 {
    //     grade = 'A';
    // } 
    // else if marks >= 80 {
    //     grade = 'B';
    // }
    // else if marks >= 70 {
    //     grade = 'c';
    // }
    // else {
    //     grade = 'F';
    // }


    let grade = if marks >= 90 {
        'A'
    } 
    else if marks >= 80 {
        'B'
    }
    else if marks >= 70 {
        'C'
    }
    else {
        'F'
    };
    // returning values from all the branches must have same data type , also "" is string and '' is char, so can't even change that 
    // returning values are withourt semicolon, if have more lines than 1, then last line withour semicolon will return

    println!("marks {marks} grade {grade}");


    // match patten - allow to compare a value against a series of patterns and 
    // execute code based on first matching pattern
    
    let marks = 95;
    let mut grade = 'N';
    
    match marks {
        90..=100 => grade = 'A',
        80..=89 => grade = 'B',
        70..=79 => grade = 'C',
        _ => grade = 'F',
    };
    
    println!("marks {marks} grade {grade}");
    
    // each matching pattern along with code block is called arm
    // e.g. 90..=100 => grade = 'A' is first arm
    // each arm has two parts -->1. pattern to match (90..100)  2. code block to execute if pattern matches
    // .. syntax is used to mention range of values in rust.. 90..=100 , starts from 90 ends at 100 (= indicates to include the last value)
    
    // Rust enforces exhaustive pattern matching, i.e. every possible value must be accounted for in the match statement -- // if all possible values are not inclued then copiler will give error of non-exhaustvie patterns
    // _ is default arm , is included for the purpose to implement exhaustive pattern matching, so if no arm match value then defaul arm executed, (_ mention everything in rust) 
    // (in above use calse grade is i32 so all possible value of i32 should be considered)
    
    // unreachable pattern following is exapmle where 70..=79 => grade = 'C' is unreachable pattern
    // match marks {
    //     90..=100 => grade = 'A',
    //     80..=89 => grade = 'B',
        //     _ => grade = 'F',
    //     70..=79 => grade = 'C',
    // }    
    
    // overlapping patterns - in bellow example code value 90 is in overlap b/w two arms
    // match marks {
    //     90..=100 => grade = 'A',
    //     80..=90 => grade = 'B',
    //     70..=79 => grade = 'C',
    //     _ => grade = 'F',
    // }
    
    
    // returning the value from match pattern 
    let grade = match marks {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        _ => 'F',
    };
    
    println!("marks {marks} grade {grade}");


}
}

mod control_flow {
pub fn loops() {

    // Loops 
    
    // loop {
    //     println!("Simple loop- executes forever unless you explicitly exit out of it");
    // }
    
    // To exit out of loop use break statement
    loop {
        println!("Simple loop- with break statment");
        break;
    }
    
    // if have nested loops, the break will only exit out of the inner loop, but not from outer loop
    // to tell compiler to exit from outer loop we use labeling.
    'outer: loop {
        println!("Simple loop- with labeling");
        break 'outer;
    }
    
    // loop as an expression with a returning value
    // in below example 5 will be return when loop breaks
    // use case of this is - if potentially have a failing operation - in such cases loop will keep attempting the operation until it succeeds then assign the resulting value to varialbe 
    let a = loop {
        break 5;
    };
    
    println!("{a}");
    

}
}






// mod conditionals;
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
    
    // covering loops / control flow
    control_flow::loops();


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
```
