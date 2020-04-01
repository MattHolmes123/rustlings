// iterators4.rs

// My original version
// pub fn factorial(num: u64) -> u64 {
//     if num == 1 {
//         return num
//     }
//     let x_iter = 2..=num;
//     let factorial = x_iter.fold(1, |acc, x| acc * x);
//     factorial
// }



pub fn factorial(num: u64) -> u64 {
    // Complete this function to return factorial of num
    // Do not use:
    // - return
    // For extra fun don't use:
    // - imperative style loops (for, while)
    // - additional variables
    // For the most fun don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    // this is what I used: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
    match num {
        1 => { num },
        _ => { (2..=num).fold(1, |acc, x| acc * x) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
