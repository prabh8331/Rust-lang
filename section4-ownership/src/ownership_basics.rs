pub fn ownership_basics() {

    // Ownership Basics

    /*
    1. Each value has a variable that's its "owner"
    2. A value can have only one owner at a time
    3. If the owner goes out of scope, the value is cleaned up
     */

     let s1 = String::from("world");
     let s2 = s1;
    //  println!("s1 is:{s1}")  // will give error

}