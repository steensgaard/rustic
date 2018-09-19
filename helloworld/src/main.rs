// Complete the function solveMeFirst to compute the sum of two integers.
// Function prototype:
// int solveMeFirst(int a, int b);
// where,
//     a is the first integer input.
//     b is the second integer input
// Return values
//     sum of the above two integers
// Sample Input
// a = 2
// b = 3
// Sample Output
// 5
// Explanation
// The sum of the two integers 2 and 3 is computed as: 5. 

fn main() {
    let summation:i32 = sum(2,3);
    println!("{}", summation);
}

pub fn sum (a:i32, b:i32) -> i32 {
    return a + b;
}

#[cfg(test)]
mod main_tests;