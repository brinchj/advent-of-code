use std::{collections::HashSet, io::stdin};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Snake {
    head: Point,
    tail: Point,
}

fn delta(target: isize, current: isize) -> isize {
    match isize::cmp(&current, &target) {
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Greater => -1,
    }
}

impl Snake {
    fn up(&mut self) {
        self.head.y += 1;
    }

    fn down(&mut self) {
        self.head.y -= 1;
    }

    fn right(&mut self) {
        self.head.x += 1;
    }

    fn left(&mut self) {
        self.head.x -= 1;
    }

    fn tail(&mut self) {
        if isize::abs_diff(self.head.x, self.tail.x) > 1 {
            self.tail.x += delta(self.head.x, self.tail.x);
            self.tail.y += delta(self.head.y, self.tail.y)
        }
        if isize::abs_diff(self.head.y, self.tail.y) > 1 {
            self.tail.y += delta(self.head.y, self.tail.y);
            self.tail.x += delta(self.head.x, self.tail.x)
        }
    }
}

fn main() {
    let mut snakes: Vec<Snake> = (0..9)
        .map(|_| Snake {
            head: Point { x: 0, y: 0 },
            tail: Point { x: 0, y: 0 },
        })
        .collect();

    let mut tail_set = HashSet::new();

    for line in stdin().lines().flatten() {
        let (left, right) = line.split_once(' ').unwrap();
        let count: i32 = right.parse().unwrap();
        for _ in 0..count {
            match left {
                "R" => snakes[0].right(),
                "U" => snakes[0].up(),
                "L" => snakes[0].left(),
                "D" => snakes[0].down(),
                _ => panic!("unexpected direction"),
            }
            snakes[0].tail();

            for i in 1..snakes.len() {
                snakes[i].head = snakes[i - 1].tail.clone();
                snakes[i].tail();
            }

            tail_set.insert(snakes[snakes.len() - 1].tail.clone());
            // println!("{:?}", snakes[snakes.len() - 1]);
        }
    }

    println!("{}", tail_set.len())
}
