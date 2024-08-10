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
    // use case of this is - if potentially have a failing operation - 
    // in such cases loop will keep attempting the operation until it succeeds then assign the resulting value to varialbe 
    let a = loop {
        break 5;
    };
    
    println!("{a}");


    // for loop - allows to loop thorugh collections such as vector or arrays etc. 

    let vec = vec![45, 30, 85, 90, 41, 39];

    for i in vec {
        println!("{i}");
    }
    
    // while loops - will continue to execute while a give condition is true

    let mut num = 0;

    while num < 10 {
        num = num + 1;
    }


}
