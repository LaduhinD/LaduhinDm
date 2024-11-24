use std::collections::HashMap;

fn sock_merchant(n: i32, ar: Vec<i32>) -> i32 {
    let mut color_count = HashMap::new();
    let mut pairs = 0;

    for sock in ar {
        let counter = color_count.entry(sock).or_insert(0);
        *counter += 1;
    }

    for &count in color_count.values() {
        pairs += count / 2;
    }

    pairs
}

fn main() {
    let n = read_line()[0];
    let ar: Vec<i32> = read_line();

    println!("{}", sock_merchant(n, ar));
}

fn read_line() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().split_whitespace()
          .map(|x| x.parse().unwrap())
          .collect()
}
