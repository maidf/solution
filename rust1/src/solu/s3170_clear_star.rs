pub fn clear_stars(s: String) -> String {
    let mut min = b'z';
    let mut res = String::new();
    let mut cnt = vec![0; 26];
    for c in s.chars() {
        if c != '*' {
            res.push(c);
            cnt[(c as u8 - b'a') as usize] += 1;
            if c as u8 <= min {
                min = c as u8;
            }
        }

        if c == '*' {
            let mut i = res.len() - 1;
            for r in res.clone().chars().rev() {
                if r == min as char {
                    res.remove(i);
                    break;
                } else {
                    i -= 1;
                }
            }
            cnt[(min - b'a') as usize] -= 1;
            while min <= b'z' && cnt[(min - b'a') as usize] == 0 {
                min += 1;
            }
        }
    }

    res
}

#[cfg(test)]
mod test {
    use crate::solu::s3170_clear_star::clear_stars;

    #[test]
    fn test() {
        let s = String::from("aaba*");
        let res = clear_stars(s);
        println!("res {res}");
        assert_eq!("aab", res)
    }

    #[test]
    fn test1() {
        let s = String::from("abc");
        let res = clear_stars(s);
        println!("res {res}");
        assert_eq!("abc", res)
    }
}
