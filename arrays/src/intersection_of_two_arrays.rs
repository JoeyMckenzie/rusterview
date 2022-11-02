/// LeetCode link: https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/674/
///
/// Given two integer arrays nums1 and nums2, return an array of their intersection.
/// Each element in the result must appear as many times as it shows in both arrays and you may return the result in any order.
fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut intersection = Vec::<i32>::new();

    // sort the arrays so we can take a parallel pointer approach
    let mut cloned_nums1 = nums1;
    let mut cloned_nums2 = nums2;

    cloned_nums1.sort_unstable();
    cloned_nums2.sort_unstable();

    let mut nums1_iterator = 0;
    let mut nums2_iterator = 0;

    while nums1_iterator < cloned_nums1.len() && nums2_iterator < cloned_nums2.len() {
        let nums1_value = cloned_nums1[nums1_iterator];
        let nums2_value = cloned_nums2[nums2_iterator];

        // in the case the values match, we have an intersection
        if nums1_value == nums2_value {
            intersection.push(nums1_value);
            nums1_iterator += 1;
            nums2_iterator += 1;
            continue;
        }

        // if the values do not match, increment accordingly
        if nums1_value < nums2_value {
            nums1_iterator += 1;
        } else {
            nums2_iterator += 1;
        }
    }

    intersection
}

#[cfg(test)]
mod intersec_should {
    use super::intersect;

    #[test]
    fn return_empty_array_when_both_arrays_are_empty() {
        // act
        let result = intersect(vec![], vec![]);

        // assert
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn return_empty_array_when_one_array_is_empty() {
        // act
        let result = intersect(vec![1, 2, 3], vec![]);

        // assert
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn return_empty_array_when_no_match_elements_between_arrays() {
        // act
        let result = intersect(vec![1, 2, 3], vec![4, 5, 6]);

        // assert
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn return_array_intersection_when_arrays_have_matching_elements() {
        let expected: Vec<i32> = vec![4, 9];

        // act
        let result = intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);

        // assert
        assert_eq!(expected, result);
    }

    #[test]
    fn return_array_intersection_when_arrays_have_duplicate_matching_elements() {
        let expected: Vec<i32> = vec![2, 2];

        // act
        let result = intersect(vec![2, 2], vec![1, 2, 2, 1]);

        // assert
        assert_eq!(expected, result);
    }
}
