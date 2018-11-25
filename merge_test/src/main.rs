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
}

fn mearge_sort(list: Vec<i32>) -> Vec<i32> {
    if list.len() == 0 || list.len() == 1 {
        return list.clone();
    } else {
        let mut result: Vec<i32> = Vec::new();

        let left = &mut mearge_sort(list[0..list.len() / 2].to_vec());
        let right = &mut mearge_sort(list[list.len() / 2..list.len()].to_vec());

        let mut left_index = 0;
        let mut right_index = 0;
        if left.len() == 0 {
            return right.clone();
        }
        if right.len() == 0 {
            return left.clone();
        }

        while left_index < left.len() && right_index < right.len() {
            if left[left_index] < right[right_index] {
                result.push(left[left_index]);
                left_index += 1;
            } else {
                result.push(right[right_index]);
                right_index += 1;
            }
            if left_index >= left.len() {
                result.append(&mut right[right_index..].to_vec())
            }
            if right_index >= right.len() {
                result.append(&mut left[left_index..].to_vec())
            }
        }

        return result;
    }
}

fn main() {
    println!("Hello, world!");
}
