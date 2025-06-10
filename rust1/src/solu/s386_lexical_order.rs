pub fn lexical_order(n: i32) -> Vec<i32> {
    let mut res = Vec::new();

    fn dfs(start: i32, n: i32, res: &mut Vec<i32>) {
        if start > n {
            return;
        }
        if start <= n {
            res.push(start);
        }
        for i in 0..10 {
            let start = start * 10 + i;
            dfs(start, n, res);
        }
    }

    for i in 1..10 {
        dfs(i, n, &mut res);
    }
    
    res
}
