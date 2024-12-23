fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
  if v1 == v2 {
      if x1 == x2 {
          return "YES".to_string();
      } else {
          return "NO".to_string();
      }
  }
  
  if (x2 - x1) % (v1 - v2) == 0 && (x2 - x1) / (v1 - v2) >= 0 {
      return "YES".to_string();
  }
  "NO".to_string()
}
fn main() {
  let x1 = 0;
  let v1 = 3;
  let x2 = 4;
  let v2 = 2;
  
  let result = kangaroo(x1, v1, x2, v2);
  println!("{}", result);
}
