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


pub fn find_target_sum_ways2(nums: Vec<i32>, target: i32) -> i32 {
    // 所有+的和为 p
    // 所有-的和为 s-p
    // 目标 t = p - (s - p)
    // t = 2p -s
    // p = (t + s) / 2
    // 从 nums 中选择一些数，装入容量为 p 的背包中，使得他们的重量和恰好为 p
    // nums[i] >= 0
    // 则 +的和 p >= 0
    // 则 (s + t) >= 0 && (s + t) % 2 == 0
    let sum = nums.iter().sum::<i32>();
    let mut p = target + sum;
    
    // 处理不可能的条件
    if p < 0 || p % 2 != 0 {
        return 0;
    }

    // 背包容量
    p /= 2;

    let len = nums.len();
    let mut f = vec![vec![0; (p + 1) as usize]; len + 1];
    f[0][0] = 1;

    // 遍历nums, 获取装入背包的数的重量x
    for (i ,&x) in nums.iter().enumerate() {
        for c in 0..=p {
            if x > c {
                // 当前的数的重量 > 容量, 不选
                f[i+1][c as usize] = f[i][c as usize];
                println!("{x} > {c} f[{}][{c}] = f[{i}][{c}]", i+1)
            } else {
                // 选, 前i+1个数能凑出和为c的数量, 总方案数 = 不选的方案数 + 前i个数能凑出 加上当前值x之后等于c 的总方案数
                f[i+1][c as usize] = f[i][c as usize] + f[i][(c - x) as usize];
                println!("f[{}][{c}] = f[{i}][{c}] + f[{i}][{}]", i+1, c-x)
            }
        } 
        println!("---------------")
    }

    f[len][p as usize]
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::solu::s494_find_target_sum_ways::find_target_sum_ways2;

    use super::find_target_sum_ways;

    #[test]
    fn test() {
        let nums = vec![1, 1, 1, 1, 1];
        let res = find_target_sum_ways(nums, 3);
        assert_eq!(5, res);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 1, 1, 1, 1];
        let res = find_target_sum_ways2(nums, 3);
        assert_eq!(5, res);
    }
}
