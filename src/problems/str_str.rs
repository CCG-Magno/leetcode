use crate::commons::Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if  needle.len() > haystack.len() {
            return -1;
        }

        let mut start = 0;
        let mut end = needle.len() -1;

        while end < haystack.len() {
            if haystack[start..=end] == needle {
                return start as i32;
            }

            else {
                start += 1;
                end += 1;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_uno() {
        let haystack = String::from("sadbutsad");
        let needle = String::from("sad");
        let expected = 0;

        assert_eq!(Solution::str_str(haystack, needle), expected);
    }
}