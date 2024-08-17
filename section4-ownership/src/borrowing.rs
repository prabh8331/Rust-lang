pub fn borrowing() {
    println!("start of borrowing");

// Borrowing rule:
    // 1- At any time, you can have either one mutable reference or any number of immutable references
    // 2- References must always be valid


   let mut vec_1 = vec![4,5,6] ; // creating a mutable vector
   let ref_vec1 = &mut vec_1; // creating a mutable reference // & (ampersand) is used to create reference
// immutable reference allows to borrow data without changing the contents
// mutable reference allow to borrow data and also change it
    
    let ref_vec2 = &mut vec_1;   

    // println!("ref1: {:?}",ref_vec1);   
    // this will give error as per borrowing rules can't have more than 1 mutable reference at a time  ( this will give error only when print statment is there) , if this println was not commented out then scope of ref_vec1 will be from its creation line to last used line i.e. println, so in that scope ref_vec2 will be come hence gives errors due to more than 1 mutable reference 

    println!("ref2: {:?}",ref_vec2); // this will work because at this time ref_vec1 scope is just in the line where it is created and scope of ref_vec2 is from its creating line to this print statment , and there is not overlap b/w scope of both referece, hence rule is not broken of 1 mutable reference only 

    // println!("ref1: {:?}, ref2: {:?}", ref_vec1, ref_vec2) ;  // on commenting this print statment code will still compile because - rust compiler counts the active period of reference, (also called scope of reference) i.e. counted from line in which it is introduced ore defined until the last line which reference is bening used

    let ref_vec3 = &vec_1;
    let ref_vec4 = &vec_1;

    println!("ref3: {:?}, ref4: {:?}", ref_vec3, ref_vec4); // this will work becuse we can have n number of immutable references at a time


    // println!("ref2: {:?}, ref3: {:?}, ref4: {:?}", ref_vec2, ref_vec3, ref_vec4); // this will error that - cannot borrow `vec_1` as immutable because it is also borrowed as mutable - at this println scople of ref_vec2 becomes from it line of creating to this line, and in this scope we are defining immutable reference, and as per the rule we can either have only one mutable reference at a time or any number of immutable reference , but not both mutable and immutable together

    // by enforcing this rule 1 rust avoids the problem of data race at compile time
    
    // data race - condition occurs when there are multiple references to the same data
    // with atlease one reference updating the data, and there is no mechanism to synchronized access to the data 

    // by enforcing the borrowing rules rust either allows reading of data through multiple immutable reference or updating or writting to data through a single mutable reference
    // this ensures that muliple parts of code can safely interact with the data without causing a race condition



    let vec_2 = { // storing the return value from the scope to vec_2
        let vec_3 = vec![1, 2, 3];  // creating vec_3
        &vec_3   // returning the refrence of the vec_3 from this scope // but at the end of this scope vec_3 will be cleaned
    } // this will give error - `vec_3` dropped here while still borrowed
    // this error is due to dangling reference inside the inner scope
    //  we created vec3, which has the ownership , then we returned a reference, and at the end of scope vec3 will dropped and cleaned up, which mwans that a reference to vec3 which returned will be a dangling reference, this is voilation of rule two
    

    println!("end of borrowing");

}