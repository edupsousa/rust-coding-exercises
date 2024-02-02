fn solution(picture: Vec<String>) -> Vec<String> {
    let line_len = picture.iter().map(|line| line.len()).max().unwrap() + 2;
    let mut bordered_picture = Vec::new();
    bordered_picture.push("*".repeat(line_len));
    for line in picture {
        bordered_picture.push(format!("*{}*", line));
    }
    bordered_picture.push("*".repeat(line_len));

    bordered_picture
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line_picture() {
        let picture = vec!["abc".to_string()];
        let expected = vec![
            "*****".to_string(),
            "*abc*".to_string(),
            "*****".to_string(),
        ];
        assert_eq!(solution(picture), expected);
    }

    #[test]
    fn test_multiple_line_picture() {
        let picture = vec!["abc".to_string(), "def".to_string()];
        let expected = vec![
            "*****".to_string(),
            "*abc*".to_string(),
            "*def*".to_string(),
            "*****".to_string(),
        ];
        assert_eq!(solution(picture), expected);
    }

}
