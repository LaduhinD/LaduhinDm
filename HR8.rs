fn birthdayCakeCandles(candles: Vec<i32>) -> i32 {
  let max_height = *candles.iter().max().unwrap();
  candles.iter().filter(|&&x| x == max_height).count() as i32
}

fn main() {
  let candles = vec![3, 2, 1, 3];
  let result = birthdayCakeCandles(candles);
  println!("{}", result);
}
