fn plus_minus(arr: Vec<i32>) {
  let mut positive = 0;
  let mut negative = 0;
  let mut zero = 0;
  for &num in &arr {
      if num > 0 {
          positive += 1;
      } else if num < 0 {
          negative += 1;
      } else {
          zero += 1;
      }
  }
  let total = arr.len() as f64;
  println!("{:.6}", positive as f64 / total);
  println!("{:.6}", negative as f64 / total);
  println!("{:.6}", zero as f64 / total);
}

fn main() {
  let arr = vec![-4, 3, -9, 0, 4, 1];
  plus_minus(arr);
}
