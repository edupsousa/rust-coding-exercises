use std::cmp::max;

fn solution(input: Vec<String>) -> Vec<String> {
    let longest = input.iter().fold(0, |acc, v| max(acc, v.len()));
    let output = input.into_iter().filter(|s| s.len() == longest).collect();
    output
}

#[cfg(test)]
mod challenges {
    #[test]
    fn longest_strings() {
        let input_str = vec!["aba", "aa", "ad", "vcd", "aba"];
        let input: Vec<String> = input_str.iter().map(|&s| s.to_string()).collect();

        let expected_str = vec!["aba", "vcd", "aba"];
        let expected: Vec<String> = expected_str.iter().map(|&s| s.to_string()).collect();

        assert_eq!(crate::solution(input), expected);
    }
}
