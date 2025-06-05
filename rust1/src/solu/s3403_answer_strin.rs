
pub fn answer_string(word: String, num_friends: i32) -> String {
    if num_friends == 1 {
        return word;
    }
    let n = word.len();
    let num = num_friends as usize;
    let m = n - (num - 1);
    let mut max = &word[0..m];
    for i in 1..n {
        let sub = &word[i..(i+m).min(n)];
        max = max.max(&sub);
    }
    max.to_string()
}


#[cfg(test)]
mod test {
    use crate::solu::s3403_answer_strin::answer_string;

    #[test]
    fn test() {
        let word = String::from("dbca");

        let num = 2;

        let max = answer_string(word, num);
        println!("max: {max}")
    }
    #[test]
    fn test1() {
        let word = String::from("gggg");

        let num = 4;

        let max = answer_string(word, num);
        println!("max: {max}")
    }
    #[test]
    fn test2() {
        let word = String::from("bif");

        let num = 2;

        let max = answer_string(word, num);
        println!("max: {max}")
    }
    #[test]
    fn test3() {
        let word = String::from("cgwzebexlahnfqsetbeaybmfdzywpvehjybpwiotnciddjonfi");

        let num = 21;

        let max = answer_string(word, num);
    
        assert_eq!(String::from("zywpvehjybpwiotnciddjonfi"), max);
        println!("max: {max}")
    }
}
