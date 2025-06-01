use std::collections::HashSet;

pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let n = is_connected.len();
    let mut res = (0..n).map(|n| n as i32).collect::<Vec<i32>>();
    for i in 0..n {
        for j in 0..n {
            if is_connected[i][j] == 1 {
                let c = res[i];
                let t = res[j];
                if c != t {
                    res.iter_mut().for_each(|r| {
                        if *r == t {
                            *r = c
                        }
                        
                    });
                }
            }
        }
    }
    let mut set = HashSet::new();
    for i in res {
        set.insert(i);
    }
    set.len() as i32
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::solu::s547_find_circle_num::find_circle_num;

    #[test]
    fn test() {
        let nums = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
        let res = find_circle_num(nums);
        assert_eq!(2, res);
    }

    #[test]
    fn test2() {
        let nums = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        let res = find_circle_num(nums);
        assert_eq!(3, res);
    }
}
