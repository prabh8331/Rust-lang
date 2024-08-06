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
        'c'
    }
    else {
        'F'
    };
// returning values from all the branches must have same data type , also "" is string and '' is char, so can't even change that 
// returning values are withourt semicolon, if have more lines than 1, then last line withour semicolon will return

    println!("marks {marks} grade {grade}");




}