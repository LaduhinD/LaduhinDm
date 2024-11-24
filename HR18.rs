fn bon_appetit(bill: Vec<i32>, k: usize, b: i32) {
  let anna_share: i32 = bill.iter().enumerate()
                             .filter(|&(i, _)| i != k)
                             .map(|(_, &cost)| cost)
                             .sum::<i32>() / 2;

  if b == anna_share {
      println!("Bon Appetit");
  } else {
      println!("{}", b - anna_share);
  }
}

fn main() {
  let first_line: Vec<i32> = read_line();
  let k = first_line[1] as usize;
  let bill: Vec<i32> = read_line();
  let b: i32 = read_line()[0];

  bon_appetit(bill, k, b);
}

fn read_line() -> Vec<i32> {
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();
  input.trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
