


```rs
// Problem 5:
// Solution: 
/*
A Pythagorean triple consists of three positive integers a, b, and c, satisfying the condition a^2 + b^2 = c^2.
These triples are commonly written as (a, b, c), and a well-known example is (3, 4, 5).

Write a program that computes the Pythagorean triplet such that a < b < c and a + b + c = 1000.
*/

// soluton 1 
fn main() {
    let mut flag = true;
    for a in 1..=1000 {
        for b in a + 1..1000 {
            // this ensures that a < b
            for c in b + 1..1000 {
                //  this ensure that b < c
                if a * a + b * b == c * c && a + b + c == 1000 {
                    println!(
                        "\n\n The required pathagorian triplet are ({}, {}, {}) \n\n",
                        a, b, c
                    );
                    flag = false;
                    break;
                }
            }
            if flag == false {
                break;
            }
        }
        if flag == false {
            break;
        }
    }
}

// solutoin 2 
fn main() {
    for a in 1..=1000 {
        for b in a + 1..1000 {
            for c in b + 1..1000 {
                if a * a + b * b == c * c && a + b + c == 1000 {
                    println!(
                        "\n\n The required pathagorian triplet are ({}, {}, {}) \n\n",
                        a, b, c
                    );

                    return;
                }
            }
        }
    }
}

// solution 3 

fn main() {
    for a in 1..=1000 {
        for b in a..=1000 - a {
            let c = 1000 - a - b;
            if a * a + b * b == c * c {
                println!("Got a triplet {:?}", (a, b, c));
            }
        }
    }
}

// solution 4 

/*
   For any two positive integers (m) and (n) where (m > n):

     [a = 2mn] [b = m^2 - n^2] [c = m^2 + n^2]

   This will give you a Pythagorean triple ((a, b, c)).
*/

const LIMITX: i32 = 25;
const SUM: i32 = 1_000;

fn gen_pyth_triples_revx() {
    for m in 3..LIMITX {
        for n in 2..LIMITX {
            // The immediate line of code below is a refactoring of (a+b+c)
            // a + b + c = 2mn + (m^2 - n^2) + (m^2 + n^2), which becomes
            //           = 2mn + (m^2) + (m^2), and translates to
            //           = 2mn + (2 * m^2), then rearranging
            //           = (2 * m^2) + 2mn, and finally
            //           = 2 * (m^2 + mn), which gives the line below
            if 2 * m * (m + n) == SUM {
                println!("`m` = {}, `n` = {}", m, n);
                println!(
                    "Required Pythagorean triplet is ({}, {}, {})",
                    2 * m * n,
                    m * m - n * n,
                    m * m + n * n
                );
                return;
            }
        }
    }
}

fn main() {
    gen_pyth_triples_revx();
}


```