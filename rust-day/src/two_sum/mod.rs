fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut num_index: HashMap<i32, i32> = HashMap::new();
    for (i, v) in nums.iter().enumerate() {
        let expected_num = target - v;
        match num_index.get(&expected_num) {
            Some(&index) => return vec![index, i as i32],
            None => {
                num_index.insert(*v, i as i32);
            }
        }
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn case1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
    #[test]
    fn case2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }
    #[test]
    fn case3() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
