use std::{collections::HashSet, hash::Hash, io::stdin};

fn count_unique<E, I>(cs: I) -> usize
where
    I: Iterator<Item = E>,
    E: Eq + Hash,
{
    let hs: HashSet<E> = HashSet::from_iter(cs);
    hs.len()
}

fn main() {
    for line in stdin().lines().flatten() {
        let cs: Vec<char> = line.chars().into_iter().collect();
        for i in 0..line.len() {
            if count_unique(cs[i..i + 14].iter()) == 14 {
                println!("{}", i + 14);
                return;
            }
        }
    }
}
