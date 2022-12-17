struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0i32;

        let mut last_char: Option<char> = None;

        for char in s.chars() {
            match char {
                'I' => {
                    result += 1;
                }
                'V' => {
                    if last_char == Some('I') {
                        result += 3;
                    } else {
                        result += 5;
                    }
                }
                'X' => {
                    if last_char == Some('I') {
                        result += 8;
                    } else {
                        result += 10;
                    }
                }
                'L' => {
                    if last_char == Some('X') {
                        result += 30;
                    } else {
                        result += 50;
                    }
                }
                'C' => {
                    if last_char == Some('X') {
                        result += 80;
                    } else {
                        result += 100;
                    }
                }
                'D' => {
                    if last_char == Some('C') {
                        result += 300;
                    } else {
                        result += 500;
                    }
                }
                'M' => {
                    if last_char == Some('C') {
                        result += 800;
                    } else {
                        result += 1000;
                    }
                }
                _ => {}
            }

            last_char = Some(char);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        let actual = Solution::roman_to_int("III".to_string());
        assert_eq!(actual, 3);

        let actual = Solution::roman_to_int("LVIII".to_string());
        assert_eq!(actual, 58);

        let actual = Solution::roman_to_int("MCMXCIV".to_string());
        assert_eq!(actual, 1994);
    }
}
