fn miniMaxSum(arr: Vec<i64>) {
  let sum: i64 = arr.iter().sum();
  let min = sum - *arr.iter().max().unwrap();
  let max = sum - *arr.iter().min().unwrap();
  println!("{} {}", min, max);
}

fn main() {
  let arr = vec![1, 2, 3, 4, 5];
  miniMaxSum(arr);
}
