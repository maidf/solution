
pub fn robot_with_string(s: String) -> String {
    let mut cnt = vec![0; 26];
    for c in s.chars() {
        cnt[(c as u8 - b'a') as usize] += 1;
    }
    
    let mut stack = Vec::new();
    let mut res = String::new();
    let mut min = b'a';
    for c in s.chars() {
        stack.push(c);
        cnt[(c as u8 - b'a') as usize] -= 1;
        
        while min < b'z' && cnt[(min - b'a') as usize] == 0 {
            min += 1;
        }
        
        while !stack.is_empty() && stack.last().unwrap() <= &(min as char) {
            res.push(stack.pop().unwrap());
        }
    }

    res
}

#[cfg(test)]
mod test {
    use crate::solu::s2434_robot_with_string::robot_with_string;

    #[test]
    fn test() {
        let s = String::from("bdda");
        let res = robot_with_string(s);
        println!("res: {res}");
        assert_eq!(String::from("addb"), res);
    }
}
