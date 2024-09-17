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
