use core::num;

fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();


    let mut res = vec![];
    
    for w in nums.windows(3).step_by(3) {
        if (w[0] - w[2]).abs() > k {
            return vec![];
        }
        res.push(w.to_vec());
    }

    res
}

#[cfg(test)]
mod test {
    use crate::solu::s2966_divide_array::divide_array;

    #[test]
    fn test() {
        let nums = vec![1, 3, 4, 8, 7, 9, 3, 5, 1];
        let k = 2;
        let res = divide_array(nums, k);

        assert_eq!(vec![vec![1, 1, 3], vec![3, 4, 5], vec![7, 8, 9]], res);
    }

    #[test]
    fn test1() {
        let nums = vec![2, 4, 2, 2, 5, 2];
        let k = 2;
        let res = divide_array(nums, k);

        assert_eq!(vec![] as Vec<Vec<i32>>, res);
    }
}
