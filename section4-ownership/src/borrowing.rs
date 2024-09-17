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

/*

    let vec_2 = { // storing the return value from the scope to vec_2
        let vec_3 = vec![1, 2, 3];  // creating vec_3
        &vec_3   // returning the refrence of the vec_3 from this scope // but at the end of this scope vec_3 will be cleaned
    } // this will give error - `vec_3` dropped here while still borrowed
    // this error is due to dangling reference inside the inner scope
    //  we created vec3, which has the ownership , then we returned a reference, and at the end of scope vec3 will dropped and cleaned up, which mwans that a reference to vec3 which returned will be a dangling reference, this is voilation of rule two
    
*/

    // Borrowing in Function
        // - At any time, you can have either one mutable reference or any number of immutable references
        // - references must always be valid
    let vec_1 = vec![1, 2, 3];
    // takes_ownership(vec_1); //  transfer of ownership from Vec1 in the main to the variable vec inside the function and if we try to access this variable after function call it gives error


    // takes_ownership(vec_1.clone()); and previously it fixed by cloning vec_1, but a cloning will make a new heap allocation and is therefore not very efficient

    // in this case it is more sensible to pass a reference to vec_1 to function because function is only printing the vector passed and it does not need to take ownership as it is not concerned with the task of determinin when vector shoup be cleaned up

    let ref1 = &vec_1;

    // let ref2 = &mut vec_1; // we can't have this mutable function because the ref2 is comming in the scope of ref1 {which is from above line to bellow line i.e. from created to last used} .. this is because we can't have immutable and mutable reference together

    borrows_vec(ref1); // it is cheaper than cloing because we are not making any new heap allocation as funciton is not taking ownership, so now vec_1 is also available in main
    println!("vec_1 is : {:?}", vec_1);


    // takes_and_gives_ownership(vec_1); here again we are passing ownership of vec_1 inside the funciton so when try to print bellow we gets error
    // println!("vec_1 is : {:?}", vec_1);


    // let vec_1 = takes_and_gives_ownership(vec_1); // we can solve above issue using shadowing because this function returns back the ownership, so returned ownership is assigend to vec_1, but it is not efficient
    // println!("vec_1 is : {:?}", vec_1);

    // achiving above task without transferring and receiving the ownership using mutable reference
    // as mutable reference can only be created for mutable variable , so lets make vec_1 mutable 
    let mut vec_1 = vec![1, 2, 3];
    let ref2 = &mut vec_1;
    mutabley_borrows_vec(ref2) ;

    println!("vec_1 is : {:?}", vec_1);



    println!("end of borrowing");

}

// fn takes_ownership(vec: Vec<i32>) {  // before passing the reference where we were passing variable itself, and this takes reference
fn borrows_vec(vec: &Vec<i32>) {  // when passing reference, need to indicate in the function's signature that it expects a reference rather than an owned value, and done using '&' .. renaming from takes_ownership to borrow_vec
    println!("vec is : {:?}", vec);
}


// fn takes_and_gives_ownership(mut vec: Vec<i32>) -> Vec<i32> { // before passing the reference
fn mutabley_borrows_vec(vec: &mut Vec<i32>) { // updating the sighature so that it accepts a mutable reference and as we don't need to pass back the ownership we remove return 
    vec.push(10);
}



// fn gives_onership() -> &Vec<i32> {
//     let vec = vec![4, 5, 7];
//     &vec
// }


// Now if instead of returning the vector, we return a reference to the vector then we will encounter an error missing lifetime specifier.

// The function return type contains a borrowed value, but there is no value for it to be borrowed from.

// In this specific case, the error arises because we are violating borrowing rule number two, which states that references must always remain valid.

// This occurs because inside the function we create the vector vec which has ownership. We then return a reference to the vector, but once the function ends, the vector is dropped and cleaned up, resulting in a dangling reference, meaning that the reference will not be pointing to valid memory.

// Generally , when we create a value within a function and intend to return it, you must transfer ownership of that value. Returning a reference is not advisable because the old value will be automatically cleaned up at the function's end, rendering the reference invalid.