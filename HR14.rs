fn breaking_records(scores: Vec<i32>) -> Vec<i32> {
  let mut min_score = scores[0];
  let mut max_score = scores[0];
  let mut min_breaks = 0;
  let mut max_breaks = 0;

  for &score in scores.iter().skip(1) {
      if score < min_score {
          min_score = score;
          min_breaks += 1;
      } else if score > max_score {
          max_score = score;
          max_breaks += 1;
      }
  }

  vec![max_breaks, min_breaks]
}

fn main() {
  let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
  let result = breaking_records(scores);
  println!("{} {}", result[0], result[1]);
}
