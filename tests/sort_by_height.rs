fn solution(mut a: Vec<i32>) -> Vec<i32> {
  let mut people: Vec<i32> = a.iter().filter(|&&v| v != -1).cloned().collect();
  people.sort();
  let mut people_iter = people.into_iter();
  
  for v in a.iter_mut() {
    if *v != -1 {
      *v = people_iter.next().unwrap();
    }
  }

  a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_already_sorted() {
        assert_eq!(solution(vec![-1, 150, 160, 170, -1, -1, 180, 190]), vec![-1, 150, 160, 170, -1, -1, 180, 190]);
    }

    #[test]
    fn test_reverse_order() {
        assert_eq!(solution(vec![-1, 190, 180, 170, -1, -1, 160, 150]), vec![-1, 150, 160, 170, -1, -1, 180, 190]);
    }

    #[test]
    fn test_multiple_trees() {
        assert_eq!(solution(vec![-1, -1, -1, 170, -1, -1, -1, 150]), vec![-1, -1, -1, 150, -1, -1, -1, 170]);
    }

    #[test]
    fn test_single_tree() {
        assert_eq!(solution(vec![-1]), vec![-1]);
    }

    #[test]
    fn test_single_person() {
        assert_eq!(solution(vec![190]), vec![190]);
    }

    #[test]
    fn test_empty() {
        assert_eq!(solution(vec![]), vec![]);
    }
}