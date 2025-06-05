pub fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    let mut res = vec![1; n];

    for i in 1..n {
        if ratings[i] > ratings[i - 1] {
            res[i] = res[i - 1] + 1;
        }
    }
    println!("res: {:?}", res);

    let mut ret = res[n-1];
    for i in (0..n-1).rev() {
        if ratings[i] > ratings[i+1] {
            res[i] = res[i].max(res[i+1] + 1);
            ret += res[i];
        } else {
            ret += res[i];
        }
    }
    
    println!("res: {:?}", res);
    
    ret
}

#[cfg(test)]
mod test {
    use super::candy;

    #[test]
    fn test() {
        let ratings = vec![1, 0, 2];
        let res = candy(ratings);
        assert_eq!(5, res)
    }

    #[test]
    fn test2() {
        let ratings = vec![1, 2, 87, 87, 87, 2, 1];
        let res = candy(ratings);
        assert_eq!(13, res)
    }
}
