fn staircase(n: i32) {
  for i in 1..=n {
      let spaces = " ".repeat((n - i) as usize);
      let hashes = "#".repeat(i as usize);
      println!("{}{}", spaces, hashes);
  }
}
fn main() {
  let n = 6;
  staircase(n);
}
