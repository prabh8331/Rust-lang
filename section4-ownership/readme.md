# OwnerShip in Rust

create the new package - 
```bash
cargo new section4-ownership 
# warning: the name `Section4-Ownership` is not snake_case or kebab-case which is recommended for package names, consider `section4-ownership`
```

The Book - official book for the rust language-
this covers all consepts very clearly -
https://doc.rust-lang.org/book/foreword.html


![memory](memory.drawio.png)

## Ownership Basics

1. Each value has a variable that's its "owner."
2. A value can have only one owner at a time. 
3. If the owner goes out of scope, the value is cleand up. 

Move the ownership
```rs

    let s1 = String::from("world");
    let s2 = s1; 
    //  println!("s1 is:{s1}")  // will give error

    // in above case under the hood this assignment is move or transfer of ownership from s1 to s2
    // "world" is a value and has variable that's it owner and can have only one owner at a time, so in line 2 s2 becomes the owner and s1 is removed from memory 
    // if try to access the s1 after the moved to s2 will give error  -- borrow of a moved value s1 (value borrowed here after move)


```
![stack_heap](ownership_basics_stack_heap.drawio.png)

Don't move the value but copy it so want we can use s1 lateron
```rs

    let s1 = String::from("world");
    let s2 = s1.clone();
    println!("s1 is:{s1}");
    
```
![clone](ownership_basics_stack_heap_clone.drawio.png)

when owner goes out of scope the value is dropped 
```rs

    let s1 = String::from("world");
    {
        let s2 = s1; 
    }
    //  println!("s2 is:{s2}")  // will give error can not find in this scope

    // in above case when inner scope ends, it will be popped out of the stack and memory it is pointing to will be immediatly freed up 
    // this prevents dangling pointers and memory leaks


```


When we assign the value of one variable to another. The value is being moved, leading to a change of ownership. However, this is not true for some primitive types.
```rs
    let x = 45;
    let y = x;
    println!("{x}");
    // this above will works because  in rust the primitives such as integers, floats, bools, and chars
    // are entirely stored on stack with no reference to heap, and they are copied and not moved by default.
    // Each copy has its separate owner.
```

## Ownership in functions 
Ownership in context of funtions works same as above, here are few examples- 

```rs

fn main() {

    // // case1- Moving ownership to the function 


    // like string are also heap allocated with a pointer length and capacity stored in stack
    let vec_1 = vec![1, 4, 5]
    takes_ownership(vec_1);
    // println!("vec_1 is: {:?}", vec_1); this will give error of borrowed of moved value
    // passing a variable has same effect as assigning a variable to another variable, i.e. ownership of vec_1 is transfered to the vec_fuc inside the funciton
    // vec_fun lives within the function scope at the end of the fuction it will be dropped (i.e. it will be cleaned up from heap)    

    // but if want to use vec_1 later, instead of passing the ownership to function insidea to vec_fun then use clone method  
    let vec_1 = vec![1, 4, 5]
    takes_ownership(vec_1.clone());
    println!("vec_1 is: {:?}", vec_1);
    // here we are not passing the ownership of the vec_1 to vec_fun but first cloning the vec_1 then sending the cloned copy to funtion - owner ship of the cloned copy is moved to vec_fun, and this ownership will be droped at the end of the function resultion in the dropping of the clone copy    


    // // case2- Moving ownership out of the function
    let vec_2 = gives_ownership();
    println!("vec_2 is: {:?}", vec_2);
    // in above case ownership is transfered from funtion to main() , and vec_2 will be dropped at the end of the main()


    // // case3 - takes and gives the ownership 
    let vec_3 = takes_and_gives_ownership(vec_2);
    // println!("vec_2 is: {:?}", vec_2);   // here we will get the error (borrowed of moved value) because we have transfred the ownership from vec_2 to function's vec_m_fun 
    println!("vec_3 is: {:?}", vec_3);
    // then this funciton returns the ownership of a variable which we are storeing in vec_3, so it is avaialble for use


    // // case4 - behaviour of stack only datatype
    let x=10;
    stack_function(x);
    println!("in main() x is : {x}");
    // unlike vactor we will be able to access the variable after passing to funtion -  because the stake only data types such as integers, floats, booleans, and chars are copied and not moved
    // so when we pass x to funtion a copy of it is created and assigned to the variable var inside the funciton 
    // these 2 variables x and var are two distinct values in stack, so update to the value inside the funciton does not impact the varialbe in main 
    


    //  imp note - taking an ownership by the function and giving it back is not something generally use in rust. Instead, use borrowing.
}


fn takes_ownership(vec_fun: Vec<i32>) {
    println!("vec_fun is: {:?}", vec)_fun;

}

fn gives_ownership() -> Vec<i32> {
    vec![4,5,6]
}

fn takes_and_gives_ownership(mut vec_m_fun: Vec<i32>) -> Vec<i32> {
    vec_m_fun.push(10);
    vec_m_fun
}

fn stack_function(mut var: i32) {
    var = 56;
    println!("in function var is : {var}");
}
```

### Practice on Ownership

```rs
// Problem 1: Fix the code below so that it compiles

fn main() {
    let s1: String = String::from("this is me, ");
    let s2: &str = "Nouman";
    some_function(s1, s2); // Something is wrong here
    // Ans 
    // some_function(s1.clone(), s2.clone());
    println!("{} {}", s1, s2);
}

fn some_function(a1: String, a2: &str) {
    println!("{} {}", a1, a2);
}

```

```rs
// Problem 2: 

/* 
Fix the code below. By solving this problem you will be able to understand 
the change of ownership inside a loop 
*/ 

fn main() {
    let mut my_vec = vec![1, 2, 3, 4, 5];
    let mut temp;

    while !my_vec.is_empty() {
        temp = my_vec; // Something wrong on this line
        // Ans - as in above code when we are doing temp = my_vec we are moving ownership of my_vec to the temp of current itteraion, so my_vec is no more avaialbe to use 
        // temp = my_vec.clone(); 
        println!("Elements in temporary vector are: {:?}", temp);


        if let Some(last_element) = my_vec.pop() { // pop() is used to remove an element from the vec
            println!("Popped element: {}", last_element);
        }
    }
}

// output 
// Elements in temporary vector are: [1, 2, 3, 4, 5]
// Popped element: 5
// Elements in temporary vector are: [1, 2, 3, 4]
// Popped element: 4
// Elements in temporary vector are: [1, 2, 3]
// Popped element: 3
// Elements in temporary vector are: [1, 2]
// Popped element: 2
// Elements in temporary vector are: [1]
// Popped element: 1

```