fn get_total_x(a: Vec<i32>, b: Vec<i32>) -> i32 {
  let mut count = 0;
  let max_b = *b.iter().max().unwrap();
  let min_a = *a.iter().min().unwrap();
  
  for num in (min_a..=max_b).step_by(min_a as usize) {
      if a.iter().all(|&x| num % x == 0) && b.iter().all(|&x| x % num == 0) {
          count += 1;
      }
  } 
  count
}
fn main() {
  let a = vec![2, 4];
  let b = vec![16, 32, 96];
  
  let result = get_total_x(a, b);
  println!("{}", result);
}
