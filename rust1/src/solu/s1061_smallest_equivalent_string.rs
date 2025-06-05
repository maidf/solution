use std::collections::HashMap;

struct UnionFind {
    parent: HashMap<char, char>,
}

impl UnionFind {
    fn new() -> UnionFind {
        let mut chars = HashMap::new();
        for i in 'a'..'z' {
            chars.insert(i, i);
        }
        UnionFind { parent: chars }
    }

    fn find(&mut self, x: char) -> char {
        let p = self.parent.get(&x).unwrap().clone();
        if p != x {
            let root = self.find(p);
            self.parent.insert(x, root);
            return root;
        }
        p
    }

    fn union(&mut self, a: char, b: char) {
        /* if a < b {
            let fb = self.find(b);
            self.parent[fb] = self.find(a);
        } else {
            let fa = self.find(a);
            self.parent[fa] = self.find(b)
        } */
        let root_max = self.find(a);
        let root_min = self.find(b);
        self.parent.insert(root_max.max(root_min), root_max.min(root_min));
    }
}

pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
    let n = s1.len();
    let mut s1 = s1.chars();
    let mut s2 = s2.chars();
    let mut u = UnionFind::new();
    for _ in 0..n {
        let a = s1.next().unwrap();
        let b = s2.next().unwrap();
        u.union(a, b);
    }

    let mut res = String::new();
    for ch in base_str.chars() {
        let w = u.find(ch);
        res.push(w);
    }
    res
}

#[cfg(test)]
mod test {
    use crate::solu::s1061_smallest_equivalent_string::smallest_equivalent_string;

    #[test]
    fn test() {
        let s1 = String::from("leetcode");
        let s2 = String::from("programs");
        let base = String::from("sourcecode");

        let res = smallest_equivalent_string(s1, s2, base);
        println!("res {res}");
        assert_eq!(String::from("aauaaaaada"), res);
    }
    #[test]
    fn test1() {
        let s1 = String::from("dfeffdfafbbebbebacbbdfcfdbcacdcbeeffdfebbdebbdafff");
        let s2 = String::from("adcdfabadbeeafeabbadcefcaabdecabfecffbabbfcdfcaaae");
        let base = String::from("myickvflcpfyqievitqtwvfpsrxigauvlqdtqhpfugguwfcpqv");

        let res = smallest_equivalent_string(s1, s2, base);
        println!("res {res}");
        assert_eq!(String::from("myiakvalapayqiavitqtwvapsrxigauvlqatqhpaugguwaapqv"), res);
    }
}
