use std::io;


fn main() {

    // variable declaration
    let mut _num_str_1 = String::new();
    let mut _num_str_2 = String::new();

    // read variables
    io::stdin().read_line(&mut _num_str_1).ok();

    io::stdin().read_line(&mut _num_str_2).ok();

    // parse integers
    let mut _num_1 : i32 = match _num_str_1.trim().parse().ok() {
        None => 0,
        Some (x) => x 
    };
    let mut _num_2 : i32 = _num_str_2.trim().parse().expect("parse error 2");

    // print the sum
    println!("{}", _num_1 + _num_2);

}

// fn aVeryBigSum() {

//  }