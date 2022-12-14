use std::{collections::HashMap, io::stdin};

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    // Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Eq, PartialEq, Hash, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn step(&self, dir: Direction) -> Option<Point> {
        let x = self.x;
        let y = self.y;
        match dir {
            // Direction::Up if y > 0 => Some(Point { x, y: y - 1 }),
            Direction::Left => Some(Point { x: x - 1, y }),
            Direction::Down => Some(Point { x, y: y + 1 }),
            Direction::Right => Some(Point { x: x + 1, y }),
        }
    }

    fn to_x(self, other: Point) -> impl Iterator<Item = Point> {
        let y = self.y;
        let start = i64::min(self.x, other.x);
        let end = i64::max(self.x, other.x);
        (start..=end).map(move |x| Point { x, y })
    }

    fn to_y(self, other: Point) -> impl Iterator<Item = Point> {
        let x = self.x;
        let start = i64::min(self.y, other.y);
        let end = i64::max(self.y, other.y);
        (start..=end).map(move |y| Point { x, y })
    }

    fn to(self, other: Point) -> Box<dyn Iterator<Item = Point>> {
        if self.x != other.x {
            Box::new(self.to_x(other))
        } else {
            Box::new(self.to_y(other))
        }
    }
}

#[derive(Copy, Clone)]
struct Cursor {
    current: Point,
}

impl Cursor {
    fn step(&self, dir: Direction) -> Option<Cursor> {
        self.current.step(dir).map(|p| Cursor { current: p })
    }
}

struct Board {
    pixels: HashMap<Point, char>,
    max_y: i64,
}

impl Board {
    fn new() -> Board {
        Board {
            pixels: HashMap::new(),
            max_y: 0,
        }
    }

    fn put(&mut self, p: Point, c: char) {
        self.pixels.insert(p, c);
        self.max_y = i64::max(self.max_y, p.y);
    }

    fn cursor(&self, p: Point) -> Cursor {
        Cursor { current: p }
    }

    fn get(&self, p: &Point) -> char {
        self.pixels.get(p).copied().unwrap_or('.')
    }

    fn draw_line(&mut self, start: Point, end: Point) {
        for p in start.to(end) {
            self.put(p, '#');
        }
    }

    fn blocked(&self, p: &Point) -> bool {
        p.y == self.max_y || self.get(p) != '.'
    }

    fn drop(&mut self, from: Point) {
        self.drop_cursor(self.cursor(from))
    }

    fn drop_cursor(&mut self, prev: Cursor) {
        match prev.step(Direction::Down) {
            None => unreachable!("can always go down"),
            Some(next) if !self.blocked(&next.current) => self.drop_cursor(next),
            Some(next) => match (next.step(Direction::Left), next.step(Direction::Right)) {
                (Some(left), _) if !self.blocked(&left.current) => self.drop_cursor(left),
                (_, Some(right)) if !self.blocked(&right.current) => self.drop_cursor(right),
                _ => {
                    self.put(prev.current, 'o');
                }
            },
        }
    }
}

fn main() {
    let mut board = Board::new();

    for line in stdin().lines().flatten() {
        let mut prev_opt = None;
        for s in line.split(" -> ") {
            let (a, b) = s.split_once(',').unwrap();
            let p = Point {
                x: a.parse().unwrap(),
                y: b.parse().unwrap(),
            };

            if let Some(prev) = prev_opt {
                board.draw_line(prev, p);
            }

            prev_opt = Some(p);
        }
    }

    board.max_y += 2;

    let start = Point { x: 500, y: 0 };
    let mut count = 0;
    while board.get(&start) != 'o' {
        count += 1;
        board.drop(start);
    }
    println!("{}", count);
}
