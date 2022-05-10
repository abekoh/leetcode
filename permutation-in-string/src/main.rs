#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut array1: [usize; 26] = [0; 26];
        let mut array2: [usize; 26] = [0; 26];
        for c in s1.chars() {
            array1[(c as u8 - b'a') as usize] += 1;
        }
        for (i, c) in (s2.as_bytes() as &[u8]).iter().enumerate() {
            array2[(c - b'a') as usize] += 1;
            if i >= s1.len() {
                array2[(s2.as_bytes()[i - s1.len()] - b'a') as usize] -= 1;
            }
            if array1 == array2 {
                return true;
            }
        }
        false
    }

    pub fn check_inclusion_old(s1: String, s2: String) -> bool {
        if s1.len() == 1 {
            return s2.contains(&s1);
        }
        if s1.len() == s2.len() {
            let mut chars1: Vec<char> = s1.chars().collect();
            chars1.sort();
            let mut chars2: Vec<char> = s2.chars().collect();
            chars2.sort();
            return chars1 == chars2;
        }
        let l = s1.len();
        // let target_c = s1.chars().nth(0).unwrap();
        for target_c in s1.chars() {
            for (i, c) in s2.chars().enumerate() {
                if c == target_c {
                    let mut v = vec![];
                    if (i as i32 - l as i32 + 1) >= 0 {
                        v.push(Self::check_inclusion(
                            s1.clone(),
                            String::from(&s2[i - l + 1..i + 1]),
                        ));
                    }
                    if i + l < s2.len() {
                        v.push(Self::check_inclusion(
                            s2.clone(),
                            String::from(&s2[i..i + l]),
                        ))
                    }
                    if v.iter().any(|&x| x == true) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::check_inclusion(String::from("ab"), String::from("eidbaooo")),
            true
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::check_inclusion(String::from("ab"), String::from("eidboaoo")),
            false
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::check_inclusion(String::from("a"), String::from("ab")),
            true
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            Solution::check_inclusion(String::from("adc"), String::from("dcda")),
            true
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(
            Solution::check_inclusion(String::from("abcdxabcde"), String::from("abcdeabcdx")),
            true
        );
    }

    #[test]
    fn example_6() {
        assert_eq!(
            Solution::check_inclusion(String::from("abcde"), String::from("eabcd")),
            true
        );
    }

    #[test]
    fn example_7() {
        assert_eq!(
            Solution::check_inclusion(String::from("rvwrk"), String::from("lznomzggwrvrkxecjaq")),
            true
        );
    }
}

// lznomzgg wrvrk xecjaq

fn main() {
    println!("leetcode template");
}
