fn a_very_big_sum(ar: Vec<i64>) -> i64 {
  ar.iter().sum()
}

fn main() {
  let ar = vec![1000000001, 1000000002, 1000000003, 1000000004, 1000000005];
  let result = a_very_big_sum(ar);
  println!("{}", result);
}
