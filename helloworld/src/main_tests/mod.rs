use super::*;
#[test]
fn sum_of_numbers_greater_that_zero() {
    assert_eq!(sum(2,3), 5);
}

#[test]
fn sum_of_zero() {
    assert_eq!(sum(0,0), 0);
}

#[test]
fn add_negative_numbers() {
    assert_eq!(sum(-1,-2), -3);
}

#[test]
fn add_negative_and_positive_numbers() {
    assert_eq!(sum(-1, 2), 1);
}

#[test]
fn add_negative_and_positive_numbers_reverse_order() {
    assert_eq!(sum(2, -1), 1);
}