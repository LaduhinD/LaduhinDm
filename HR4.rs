fn diagonal_difference(arr: Vec<Vec<i32>>) -> i32 {
  let n = arr.len();
  let mut primary_diagonal_sum = 0;
  let mut secondary_diagonal_sum = 0;
  for i in 0..n {
      primary_diagonal_sum += arr[i][i];
      secondary_diagonal_sum += arr[i][n - 1 - i];
  }

  (primary_diagonal_sum - secondary_diagonal_sum).abs()
}
fn main() {
  let n = 3;
  let arr = vec![
      vec![11, 2, 4],
      vec![4, 5, 6],
      vec![10, 8, -12],
  ];
  let result = diagonal_difference(arr);
  println!("{}", result);
}
