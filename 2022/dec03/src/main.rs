use std::{collections::HashSet, io::stdin};

fn item_priority(c: char) -> u32 {
    if ('a'..='z').contains(&c) {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 1 + 26
    }
}

fn main() {
    let mut sum = 0;

    let mut lines = stdin().lines();

    while let Some(Ok(line)) = lines.next() {
        let group = vec![
            line,
            lines.next().unwrap().unwrap(),
            lines.next().unwrap().unwrap(),
        ];
        let mut sets: Vec<HashSet<char>> = group
            .into_iter()
            .map(|x| x.chars().into_iter().collect())
            .collect();
        let init = sets.pop().unwrap();
        let fset: HashSet<char> = sets.into_iter().fold(init, |a, b| {
            a.intersection(&b).into_iter().copied().collect()
        });
        if fset.len() != 1 {
            panic!("expected exactly 1 badge item")
        }
        let badge = fset.iter().next().unwrap();
        sum += item_priority(*badge);
    }

    println!("{}", sum)
}
