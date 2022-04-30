struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut result_list = Vec::<char>::new();
        let mut temp_list = Vec::<char>::new();
        for c in s.chars() {
            if temp_list.contains(&c) {
                if temp_list.len() > result_list.len() {
                    result_list = temp_list.clone();
                }
                if *temp_list.last().unwrap() == c {
                    temp_list = vec![c];
                } else {
                    let another_char_first = temp_list.iter().position(|&x| x == c).unwrap() + 1;
                    temp_list = temp_list[another_char_first..].to_vec();
                    temp_list.push(c);
                }
            } else {
                temp_list.push(c);
            }
        }
        if temp_list.len() > result_list.len() {
            temp_list.len() as i32
        } else {
            result_list.len() as i32
        }
    }
}

fn main() {
    println!(
        "{}",
        Solution::length_of_longest_substring(String::from("abcabcbb"))
    );
    println!(
        "{}",
        Solution::length_of_longest_substring(String::from("bbbbb"))
    );
    println!(
        "{}",
        Solution::length_of_longest_substring(String::from("pwwkew"))
    );
    println!(
        "{}",
        Solution::length_of_longest_substring(String::from(" "))
    );
    println!(
        "{}",
        Solution::length_of_longest_substring(String::from("dvdf"))
    );
    println!(
        "{}",
        Solution::length_of_longest_substring(String::from("aabaab!bb"))
    );
    println!(
        "{}",
        Solution::length_of_longest_substring(String::from("ohvhjdml"))
    );
}
