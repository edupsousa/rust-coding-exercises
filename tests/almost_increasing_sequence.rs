
fn solution(sequence: Vec<i32>) -> bool {
    let mut removed = false;
    for i in 0..sequence.len() - 1 {
        if sequence[i] >= sequence[i + 1] {
            if removed {
                return false;
            }
            if i > 0 && sequence[i - 1] >= sequence[i + 1] && i + 2 < sequence.len() && sequence[i] >= sequence[i + 2] {
                return false;
            }
            removed = true;
        }
    }
    true
}

mod challenges {
    use crate::solution;

  #[test]
  fn almost_increasing_sequence() {
    assert_eq!(solution(vec![1, 3, 2, 1]), false);
    assert_eq!(solution(vec![1, 3, 2]), true);
    assert_eq!(solution(vec![1, 2, 1, 2]), false);
    assert_eq!(solution(vec![3, 6, 5, 8, 10, 20, 15]), false);
  }
}
