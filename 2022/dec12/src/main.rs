use std::{
    collections::{BinaryHeap, HashMap},
    io::stdin,
};

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug, Ord, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

struct Searcher {
    terrain: Vec<Vec<char>>,
    start: Point,
    goal: Point,
}

impl Searcher {
    fn new(terrain: Vec<Vec<char>>) -> Searcher {
        let mut start = None;
        let mut goal = None;

        for (y, row) in terrain.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                match col {
                    'S' => start = Some(Point { x, y }),
                    'E' => goal = Some(Point { x, y }),
                    _ => continue,
                }
            }
        }

        Searcher {
            terrain,
            start: start.expect("no S found"),
            goal: goal.expect("no E found"),
        }
    }

    fn step(&self, p: Point, deltay: i32, deltax: i32) -> Option<Point> {
        match (deltay, deltax) {
            (1, 0) if p.y + 1 < self.terrain.len() => Some(Point { y: p.y + 1, x: p.x }),
            (-1, 0) if p.y > 0 => Some(Point { y: p.y - 1, x: p.x }),
            (0, 1) if p.x + 1 < self.terrain[0].len() => Some(Point { y: p.y, x: p.x + 1 }),
            (0, -1) if p.x > 0 => Some(Point { y: p.y, x: p.x - 1 }),
            _ => None,
        }
    }

    fn shortest_path_to_goal(&self) -> Option<usize> {
        let mut active = BinaryHeap::new();
        active.push((0, 0, b'a', self.start));

        let mut visited = HashMap::new();
        visited.insert(self.start, 0);

        self.shortest_path_to_goal_inner(&mut active, &mut visited)
    }

    fn shortest_path_to_goal_inner(
        &self,
        active: &mut BinaryHeap<(usize, usize, u8, Point)>,
        visited: &mut HashMap<Point, usize>,
    ) -> Option<usize> {
        while let Some((_, steps, current_height, current)) = active.pop() {
            // println!("{:?}, {}, {}", current, steps, current_height);

            let candidates = [
                self.step(current, -1, 0),
                self.step(current, 1, 0),
                self.step(current, 0, -1),
                self.step(current, 0, 1),
            ];

            for next in candidates.iter().flatten() {
                match self.terrain[next.y][next.x] {
                    'E' if b'z' <= current_height + 1 => return Some(steps + 1),
                    'E' => continue,
                    c if c as u8 > current_height + 1 => continue,
                    c if !visited.contains_key(next) => {
                        visited.insert(*next, steps + 1);
                        active.push((usize::MAX - (steps + 1), steps + 1, c as u8, *next))
                    }
                    c if visited.contains_key(next) => {
                        if visited[next] > steps + 1 {
                            panic!("saw faster path")
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        None
    }
}

fn main() {
    let terrain: Vec<Vec<char>> = stdin()
        .lines()
        .flatten()
        .map(|l| l.chars().collect())
        .collect();

    let mut searcher = Searcher::new(terrain);
    let mut shortest = searcher.shortest_path_to_goal().unwrap();

    for (y, row) in searcher.terrain.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'a' {
                searcher.start = Point { y, x };
                if let Some(steps) = searcher.shortest_path_to_goal() {
                    shortest = usize::min(shortest, steps)
                }
            }
        }
    }

    println!("{}", shortest)
}
