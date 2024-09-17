use std::vec;

pub fn compound_data_types() {
    // &str (string slice) 
    let fixed_str: &str = "Fixed lenget string";
    println!("fixed_str: {fixed_str}");

    // String - this comes from the rust standard library 
    // (:: is used to access accociated functions or methods on a type or module)
    let mut flexible_str: String = String::from("This sring will grow");

    // string slice is immutable i.e. once created cannot add or remove some text from it
    // String type allow text to be modified and grow in size
    println!("flexible string: {flexible_str}");
    flexible_str.push('s');
    println!("flexible string: {flexible_str}");

    // Arrays - hold multiples values ofthe same type, are fixed size, indexes starts from zero in rust
    let mut array_1 = [6 ,4 ,5 , 7, 8];
    let numx= array_1[0];
    println!("{}",array_1[1]);
    println!("{numx}");
    println!("{:?}", array_1); // {:?} is format specifier to print full compound datatype
    
    let array2 = [0;10];
    println!("{:?}", array2);

    // Vectors - counter part to arrays which can shrink and grow is called Vector
    let vec_1: Vec<i32> = vec![4, 5, 3, 2, 5,2];
    println!("{:?}", vec_1);

    let numv1= vec_1[3]; 
    println!("{}", numv1);

    // Tuples - can hold values of different types
    let my_info = ("Salary", 40000, "Age",40);
    println!("{:?}", my_info);

    let salary_value = my_info.1; // index into tuple
    println!("{}", salary_value);

    let (salary, salary_value  , age , age_value ) = my_info; //to destructure the entire tuple
    println!("{:?}",my_info);

    // empty tupel also called the unit type (not commpound )
    // they are returned implicitly when no other meaninfull value can be returned
    // e.g. functions that lack specific return value implicitly return the unit type
    // they are zero sized and do not consume any value
    let unit_tuple = ();
    println!("{:?}", unit_tuple);


}