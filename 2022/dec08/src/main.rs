use std::{collections::HashMap, io::stdin};

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone)]
struct Cursor {
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
}

impl Cursor {
    fn get(&self, x: isize, y: isize) -> Option<Cursor> {
        if (0..=self.max_x as isize).contains(&x) && (0..=self.max_y as isize).contains(&y) {
            Some(Cursor {
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
                max_x: self.max_x,
                max_y: self.max_y,
            })
        } else {
            None
        }
    }

    fn next(&self) -> Option<Cursor> {
        self.get(self.x as isize + 1, self.y as isize)
            .or_else(|| self.get(0, self.y as isize + 1))
    }

    fn step(&self, dir: Direction) -> Option<Cursor> {
        match dir {
            Direction::Up => self.get(self.x as isize, self.y as isize - 1),
            Direction::Down => self.get(self.x as isize, self.y as isize + 1),
            Direction::Left => self.get(self.x as isize - 1, self.y as isize),
            Direction::Right => self.get(self.x as isize + 1, self.y as isize),
        }
    }
}

struct Forest {
    trees: Vec<Vec<usize>>,
    tallest: Vec<Vec<HashMap<Direction, usize>>>,
}

impl Forest {
    fn new(trees: Vec<Vec<usize>>) -> Forest {
        let tallest = trees
            .iter()
            .map(|l| l.iter().map(|_| HashMap::new()).collect())
            .collect();
        Forest { trees, tallest }
    }

    fn tallest_tree_in_direction(&mut self, c: Cursor, dir: Direction) -> usize {
        if let Some(height) = self.tallest[c.y][c.x].get(&dir) {
            return *height;
        }
        let mut cursor = c.clone();
        let mut check_tallest = 0;
        while let Some(c) = cursor.step(dir) {
            cursor = c;
            check_tallest = usize::max(check_tallest, self.trees[c.y][c.x]);
        }

        let mut height = c
            .step(dir)
            .map(|next| self.tallest_tree_in_direction(next, dir))
            .unwrap_or(0);

        assert_eq!(check_tallest, height);

        height = usize::max(self.trees[c.y][c.x], height);

        self.tallest[c.y][c.x].insert(dir, height);
        height
    }

    fn visibility_in_direction(&self, mut c: Cursor, dir: Direction) -> usize {
        let mut visibility = 0;
        let height = self.trees[c.y][c.x];
        while let Some(cursor) = c.step(dir) {
            c = cursor;
            visibility += 1;
            if self.trees[c.y][c.x] >= height {
                break;
            }
        }
        visibility
    }

    fn visibility(&self, c: Cursor) -> usize {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .map(|dir| self.visibility_in_direction(c, *dir))
        .fold(1, |a, b| a * b)
    }

    fn smallest(&mut self, c: Cursor) -> usize {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .map(|dir| {
            c.step(*dir)
                .map(|c| self.tallest_tree_in_direction(c, *dir))
                .unwrap_or(0)
        })
        .min()
        .unwrap()
    }

    fn is_visible(&mut self, c: Cursor) -> bool {
        if c.x == 0 || c.y == 0 || c.x == c.max_x || c.y == c.max_y {
            true
        } else {
            let smallest_sourrounding = self.smallest(c);
            self.trees[c.y][c.x] > smallest_sourrounding
        }
    }
}

fn main() {
    let mut forest_data = vec![];
    for line in stdin().lines().flatten() {
        let trees: Vec<usize> = line
            .chars()
            .flat_map(|x| str::parse(&format!("{}", x)))
            .collect();
        forest_data.push(trees);
    }

    let mut cursor = Cursor {
        x: 0,
        y: 0,
        max_x: forest_data[0].len() - 1,
        max_y: forest_data.len() - 1,
    };
    let mut forest = Forest::new(forest_data);

    let mut visible_count = if forest.is_visible(cursor) { 1 } else { 0 };
    let mut max_scenic_score = forest.visibility(cursor);

    while let Some(c) = cursor.next() {
        cursor = c;

        max_scenic_score = usize::max(max_scenic_score, forest.visibility(c));

        if forest.is_visible(c) {
            visible_count += 1;
        }
    }

    println!();
    println!("visible trees: {}", visible_count);
    println!("max scenic score: {}", max_scenic_score);
}
