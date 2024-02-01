use std::collections::HashSet;

fn solution(matrix: Vec<Vec<i32>>) -> i32 {
  let mut sum = 0;
  let mut dead_cols = HashSet::new();
  
  for (i, row) in matrix.iter().enumerate() {
    for (j, &cell) in row.iter().enumerate() {
      if i == 0 || !dead_cols.contains(&j) {
        sum += cell;
      }
      if cell == 0 {
        dead_cols.insert(j);
      }
    }
  }

  return sum;
}

mod challenges {
    use crate::solution;

  #[test]
  fn haunted_matrix_sum() {
    let matrix = vec![
      vec![0, 1, 1, 2], 
      vec![0, 5, 0, 0], 
      vec![2, 0, 3, 3],
    ];
  
    assert_eq!(solution(matrix), 9);
  }
}