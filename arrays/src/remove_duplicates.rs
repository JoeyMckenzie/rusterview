/// LeetCode link: https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/727/
///
/// Given an integer array nums sorted in non-decreasing order, remove the duplicates
/// in-place such that each unique element appears only once.
/// The relative order of the elements should be kept the same.
///
/// Since it is impossible to change the length of the array in some languages,
/// you must instead have the result be placed in the first part of the array nums.
/// More formally, if there are k elements after removing the duplicates, then the
/// first k elements of nums should hold the final result. It does not matter what you leave beyond the first k elements.
///
/// Return k after placing the final result in the first k slots of nums.
///
/// Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.
fn remove_duplicates(mut nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    if nums.len() == 1 {
        return 1;
    }

    let mut previous_value = nums[0];
    let mut current_insert_index = 1;

    (1..nums.len()).for_each(|current_index| {
        let current_value = nums[current_index];

        if current_value != previous_value {
            nums[current_index] = current_value;
            current_insert_index += 1;
        }

        previous_value = current_value;
    });

    current_insert_index
}

#[cfg(test)]
mod remove_duplicates_should {
    use super::remove_duplicates;

    #[test]
    fn return_zero_when_no_elements_are_given() {
        // arrange
        let expected = 0_i32;

        // act
        let result = remove_duplicates(Vec::<i32>::new());

        // assert
        assert_eq!(expected, result);
    }

    #[test]
    fn return_one_when_only_one_value_exists() {
        // arrange
        let expected = 1_i32;

        // act
        let result = remove_duplicates(vec![1_i32]);

        // assert
        assert_eq!(expected, result);
    }

    #[test]
    fn returns_index_of_last_duplicated_value() {
        // arrange
        let expected = 5_i32;
        let input: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];

        // act
        let result = remove_duplicates(input);

        // assert
        assert_eq!(expected, result);
    }
}
