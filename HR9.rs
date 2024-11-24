fn time_conversion(s: &str) -> String {
  let period = &s[8..];
  let mut hours: i32 = s[0..2].parse().unwrap();

  if period == "AM" {
      if hours == 12 {
          hours = 0;
      }
  } else if period == "PM" {
      if hours != 12 {
          hours += 12;
      }
  }

  format!("{:02}:{:02}:{:02}", hours, &s[3..5], &s[6..8])
}

fn main() {
  let time = "07:05:45PM";
  let result = time_conversion(time);
  println!("{}", result);
}
