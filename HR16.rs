fn divisible_sum_pairs(n: usize, k: i32, ar: Vec<i32>) -> i32 {
  let mut count = 0;
  for i in 0..n {
      for j in i+1..n {
          if (ar[i] + ar[j]) % k == 0 {
              count += 1;
          }
      }
  }
  count
}

fn main() {
  let first_line: Vec<i32> = read_line();
  let n = first_line[0] as usize;
  let k = first_line[1];
  let ar: Vec<i32> = read_line();

  println!("{}", divisible_sum_pairs(n, k, ar));
}

fn read_line() -> Vec<i32> {
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();
  input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect()
}
