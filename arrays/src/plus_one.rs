/// LeetCode link: https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/559/
///
/// You are given a large integer represented as an integer array digits,
/// where each digits[i] is the ith digit of the integer. The digits are
/// ordered from most significant to least significant in left-to-right order.
/// The large integer does not contain any leading 0's.
///
/// Increment the large integer by one and return the resulting array of digits.
///
/// Example 1:
///
/// Input: digits = [1,2,3]
/// Output: [1,2,4]
/// Explanation: The array represents the integer 123.
/// Incrementing by one gives 123 + 1 = 124.
/// Thus, the result should be [1,2,4].
///
/// Example 2:
///
/// Input: digits = [4,3,2,1]
/// Output: [4,3,2,2]
/// Explanation: The array represents the integer 4321.
/// Incrementing by one gives 4321 + 1 = 4322.
/// Thus, the result should be [4,3,2,2].
///
/// Example 3:
///
/// Input: digits = [9]
/// Output: [1,0]
/// Explanation: The array represents the integer 9.
/// Incrementing by one gives 9 + 1 = 10.
/// Thus, the result should be [1,0].
///
/// Constraints:
///
/// 1 <= digits.length <= 100
/// 0 <= digits[i] <= 9
/// digits does not contain any leading 0's.
fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    if digits.is_empty() {
        return digits;
    }

    let cardinality = digits.len() as u32;

    let mut rolling_sum = 0_i32;

    // begin incrementing from the end of the array
    (0..digits.len() - 1).for_each(|current_index| {
        let current_value = digits[current_index];
        let magnitude = 10_i32.pow(cardinality - current_index as u32 + 1_u32);
        rolling_sum += current_value * magnitude;
    });

    // once we've build our rolling sum, increment it
    rolling_sum += 1;

    let mut digits_parsed = Vec::<i32>::new();

    while rolling_sum > 0 {
        let remainder = rolling_sum % 10;
        digits_parsed.push(remainder);
        rolling_sum /= 10;
    }

    digits_parsed.reverse();
    digits_parsed
}

#[cfg(test)]
mod plus_one_should {
    use super::plus_one;

    #[test]
    fn increment_by_one_correctly_when_no_carrying_is_required() {
        // arrange
        let expected: Vec<i32> = vec![1, 2, 4];

        // act
        let result = plus_one(vec![1, 2, 3]);

        // assert
        assert_eq!(expected, result);
    }

    #[test]
    fn increment_by_one_correctly_when_carrying_is_required() {
        // arrange
        let expected: Vec<i32> = vec![1, 2, 9];

        // act
        let result = plus_one(vec![1, 3, 0]);

        // assert
        assert_eq!(expected, result);
    }

    #[test]
    fn increment_by_one_correctly_when_carrying_is_required_with_additional_digit() {
        // arrange
        let expected: Vec<i32> = vec![1, 0, 0];

        // act
        let result = plus_one(vec![9, 9]);

        // assert
        assert_eq!(expected, result);
    }
}
