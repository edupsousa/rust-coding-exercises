fn solution(n: i32) -> bool {
  let digits = n.to_string();
  let mid = digits.len() / 2;
  let (left, right) = digits.split_at(mid);
  let left_sum = left.chars().fold(0, |acc, c| acc + c.to_digit(10).unwrap());
  let right_sum = right.chars().fold(0, |acc, c| acc + c.to_digit(10).unwrap());

  left_sum == right_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lucky_number() {
        assert_eq!(solution(123321), true);
        assert_eq!(solution(1111), true);
        assert_eq!(solution(1230), true);
    }

    #[test]
    fn test_unlucky_number() {
        assert_eq!(solution(1234), false);
        assert_eq!(solution(123456), false);
        assert_eq!(solution(1235), false);
    }
}