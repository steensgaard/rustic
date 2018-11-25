#[cfg(test)]
mod mearge {
    use super::*;

    #[test]
    fn empty_input_is_empty() {
        let input: Vec<i32> = Vec::new();
        let output: Vec<i32> = Vec::new();
        assert_eq!(output, mearge_sort(input));
    }
    #[test]
    fn input_with_one_element_return_output_with_one_element() {
        let mut input: Vec<i32> = Vec::new();
        input.push(1);
        let mut output: Vec<i32> = Vec::new();
        output.push(1);
        assert_eq!(output, mearge_sort(input));
    }

    #[test]
    fn mearge_sort_sorts_vec_with_2_elements_already_sorted() {
        let mut input: Vec<i32> = Vec::new();
        input.push(1);
        input.push(2);
        let mut output: Vec<i32> = Vec::new();
        output.push(1);
        output.push(2);
        assert_eq!(output, mearge_sort(input));
    }

    #[test]
    fn mearge_sort_sorts_vec_with_2_elements_unsorted() {
        let mut input: Vec<i32> = Vec::new();
        input.push(2);
        input.push(1);
        let mut output: Vec<i32> = Vec::new();
        output.push(1);
        output.push(2);
        assert_eq!(output, mearge_sort(input));
    }

    #[test]
    fn mearge_sort_sorts_long_unsorted_vec() {
        let mut input: Vec<i32> = Vec::new();
        input.push(10);
        input.push(8);
        input.push(2);
        input.push(4);
        input.push(6);
        input.push(5);
        input.push(7);
        input.push(3);
        input.push(9);
        input.push(1);
        let mut output: Vec<i32> = Vec::new();
        output.push(1);
        output.push(2);
        output.push(3);
        output.push(4);
        output.push(5);
        output.push(6);
        output.push(7);
        output.push(8);
        output.push(9);
        output.push(10);
        assert_eq!(output, mearge_sort(input));
    }

    #[test]
    fn mearge_sort_can_handle_negative_numbers() {
        let mut input: Vec<i32> = Vec::new();
        input.push(10);
        input.push(8);
        input.push(2);
        input.push(4);
        input.push(-6);
        input.push(5);
        input.push(7);
        input.push(-3);
        input.push(9);
        input.push(1);
        let mut output: Vec<i32> = Vec::new();
        output.push(-6);
        output.push(-3);
        output.push(1);
        output.push(2);
        output.push(4);
        output.push(5);
        output.push(7);
        output.push(8);
        output.push(9);
        output.push(10);
        assert_eq!(output, mearge_sort(input));
    }

    #[test]
    fn mearge_sort_long_sorted_list() {
        let mut input: Vec<i32> = Vec::new();
        input.push(-6);
        input.push(-3);
        input.push(1);
        input.push(2);
        input.push(4);
        input.push(5);
        input.push(7);
        input.push(8);
        input.push(9);
        input.push(10);
        let mut output: Vec<i32> = Vec::new();
        output.push(-6);
        output.push(-3);
        output.push(1);
        output.push(2);
        output.push(4);
        output.push(5);
        output.push(7);
        output.push(8);
        output.push(9);
        output.push(10);
        assert_eq!(output, mearge_sort(input));
    }

    #[test]
    fn mearge_sort_long_reversed_sorted_list() {
        let mut input: Vec<i32> = Vec::new();
        input.push(10);
        input.push(9);
        input.push(8);
        input.push(7);
        input.push(5);
        input.push(4);
        input.push(2);
        input.push(1);
        input.push(-3);
        input.push(-6);
        let mut output: Vec<i32> = Vec::new();
        output.push(-6);
        output.push(-3);
        output.push(1);
        output.push(2);
        output.push(4);
        output.push(5);
        output.push(7);
        output.push(8);
        output.push(9);
        output.push(10);
        assert_eq!(output, mearge_sort(input));
    }
}

fn mearge_sort(list: Vec<i32>) -> Vec<i32> {
    if list.len() < 2 {
        return list.clone();
    } else {
        let left = mearge_sort(list[0..list.len() / 2].to_vec());
        let right = mearge_sort(list[list.len() / 2..list.len()].to_vec());
        return mearge(left, right);
    }
}

fn mearge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut left_index = 0;
    let mut right_index = 0;
    while left_index < left.len() && right_index < right.len() {
        if left[left_index] < right[right_index] {
            result.push(left[left_index]);
            left_index += 1;
        } else {
            result.push(right[right_index]);
            right_index += 1;
        }
        if left_index >= left.len() {
            let mut rest = right[right_index..].to_vec();
            result.append(&mut rest)
        }
        if right_index >= right.len() {
            let mut rest = left[left_index..].to_vec();
            result.append(&mut rest)
        }
    }
    return result;
}

fn main() {
    println!("Hello World!!!");
}
