mod ownership_basics;

mod borrowing;

fn main() {
    println!("start of main");

    // covers the ownership_basics
    ownership_basics::ownership_basics();
    
    // covers the borrowing basics 
    borrowing::borrowing();

    
    println!("end of main");

}

