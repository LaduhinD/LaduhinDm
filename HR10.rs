fn grading_students(grades: Vec<i32>) -> Vec<i32> {
  grades.into_iter().map(|grade| {
      if grade >= 38 {
          let next_multiple_of_5 = (grade / 5 + 1) * 5;
          if next_multiple_of_5 - grade < 5 {
              next_multiple_of_5
          } else {
              grade
          }
      } else {
          grade
      }
  }).collect()
}

fn main() {
  let grades = vec![73, 67, 38, 33];
  let result = grading_students(grades);
  for grade in result {
      println!("{}", grade);
  }
}
