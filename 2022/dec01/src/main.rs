use std::{cmp::Ordering, io::stdin};

fn main() {
    let mut top3 = vec![0, 0, 0];
    let mut current = 0u32;

    for line in stdin().lines().flatten() {
        if let Ok(calories) = str::parse::<u32>(&line) {
            current += calories
        } else {
            top3.push(current);
            top3.sort_by(|a, b| {
                if *a < *b {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });
            top3.pop();
            current = 0;
        }
    }

    println!("{}", top3.iter().sum::<u32>());
}
