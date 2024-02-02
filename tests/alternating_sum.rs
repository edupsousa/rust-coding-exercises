fn solution(a: Vec<i32>) -> Vec<i32> {
  let mut team1 = 0;
  let mut team2 = 0;
  for (i, &x) in a.iter().enumerate() {
    if i % 2 == 0 {
      team1 += x;
    } else {
      team2 += x;
    }
  }
  
  vec![team1, team2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vector() {
        assert_eq!(solution(vec![]), vec![0, 0]);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(solution(vec![5]), vec![5, 0]);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(solution(vec![5, 3]), vec![5, 3]);
    }

    #[test]
    fn test_multiple_elements() {
        assert_eq!(solution(vec![5, 3, 2]), vec![7, 3]);
    }

    #[test]
    fn test_negative_elements() {
        assert_eq!(solution(vec![5, -3, 2]), vec![7, -3]);
    }

    #[test]
    fn test_zero_elements() {
        assert_eq!(solution(vec![0, 0, 0, 0]), vec![0, 0]);
    }
}