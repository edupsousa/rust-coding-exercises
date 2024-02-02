fn solution(input: String) -> String {
    let mut stack: Vec<char> = Vec::new();

    for c in input.chars() {
        if c == ')' {
            let mut to_reverse = Vec::new();
            while let Some(top) = stack.pop() {
                if top == '(' {
                    break;
                } else {
                    to_reverse.push(top);
                }
            }
            for &ch in &to_reverse {
                stack.push(ch);
            }
        } else {
            stack.push(c);
        }
    }

    stack.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(solution("".to_string()), "".to_string());
    }

    #[test]
    fn test_no_parentheses() {
        assert_eq!(solution("abcd".to_string()), "abcd".to_string());
    }

    #[test]
    fn test_single_parentheses() {
        assert_eq!(solution("(abcd)".to_string()), "dcba".to_string());
    }

    #[test]
    fn test_multiple_parentheses() {
        assert_eq!(solution("(abcd)(efgh)".to_string()), "dcbahgfe".to_string());
    }

    #[test]
    fn test_nested_parentheses() {
        assert_eq!(solution("(ab(cd)ef)".to_string()), "fecdba".to_string());
    }

    #[test]
    fn test_mixed() {
        assert_eq!(solution("a(bc)d".to_string()), "acbd".to_string());
    }
}