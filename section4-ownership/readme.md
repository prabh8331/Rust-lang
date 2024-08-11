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

```rs

    let s1 = String::from("world");
    let s2 = s1;

    // in above case under the hood this assignment is move or transfer of ownership from s1 to s2
    // "world" is a value and has variable that's it owner and can have only one owner at a time, so in line 2 s2 becomes the owner and s1 is removed from memory 
    // if try to access the s1 after the moved to s2 will give error  -- borrow of a moved value s1 (value borrowed here after move)


```

| stage | Stack | Heap |
| --- | --- | --- |
| let s1 = String::from("world"); |
