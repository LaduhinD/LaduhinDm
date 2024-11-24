fn page_count(n: i32, p: i32) -> i32 {
  let from_front = p / 2;
  let from_back = (n / 2) - (p / 2);
  std::cmp::min(from_front, from_back)
}

fn main() {
  let n = read_line()[0];
  let p = read_line()[0];
  println!("{}", page_count(n, p));
}

fn read_line() -> Vec<i32> {
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();
  input.trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
