use std::collections::VecDeque;

pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let mut sum = VecDeque::new();
    sum.push_back(0);
    let mut res = 0;
    for (i, &val) in nums.iter().enumerate() {
        let len = sum.len();
        for _ in 0..len {
            let s = sum.pop_front().unwrap();
            let l = s + val;
            let r = s - val;
            if i == nums.len() - 1 {
                if l == target {
                    res += 1;
                }
                if r == target {
                    res += 1;
                }
                continue;
            }
            sum.push_back(l);
            sum.push_back(r);
        }
    }
    res
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::find_target_sum_ways;

    #[test]
    fn test() {
        let nums = vec![1, 1, 1, 1, 1];
        let res = find_target_sum_ways(nums, 3);
        assert_eq!(5, res);
    }
}
