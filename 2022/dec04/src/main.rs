use std::io::stdin;

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && other.end <= self.end
    }
    fn has_overlap(&self, other: &Range) -> bool {
        !(self.end < other.start || other.end < self.start)
    }
}

fn build_range(fromto: &str) -> Range {
    let mut ns = fromto.split('-');
    let start = str::parse(ns.next().unwrap()).unwrap();
    let end = str::parse(ns.next().unwrap()).unwrap();
    Range { start, end }
}

fn main() {
    let mut count = 0;

    for line in stdin().lines().flatten() {
        let pair: Vec<&str> = line.split(',').collect();
        let left = build_range(pair[0]);
        let right = build_range(pair[1]);
        if left.has_overlap(&right) {
            count += 1;
        }
    }

    println!("{}", count)
}
