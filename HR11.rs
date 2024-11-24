fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: Vec<i32>, oranges: Vec<i32>) {
  let apple_count = apples.iter().filter(|&&d| a + d >= s && a + d <= t).count();
  let orange_count = oranges.iter().filter(|&&d| b + d >= s && b + d <= t).count();
  
  println!("{}", apple_count);
  println!("{}", orange_count);
}

fn main() {
  let s = 7;
  let t = 11;
  let a = 5;
  let b = 15;
  let apples = vec![-2, 2, 1];
  let oranges = vec![5, -6];
  
  count_apples_and_oranges(s, t, a, b, apples, oranges);
}
