fn main() {
    println!("{}", simple_array_sum(vec![1, 2, 3, 4, 5]));
}

fn simple_array_sum(arr:Vec<i32>) -> i32 {
 //the two methods make the same result, the second one is the functional approach
    // let sum = 0;
    // for number in arr{
    //     sum += number
    // }
    // return sum;
    return arr.iter().fold(0, |sum, val| return sum + val);
}
