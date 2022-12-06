use std::io::stdin;

const RPS_ABC: &[char] = &['A', 'B', 'C'];

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum GAME {
    LOSS,
    DRAW,
    WIN,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HAND {
    ROCK,
    PAPER,
    SCISSORS,
}

fn idx_to_hand(i: usize) -> HAND {
    match i {
        0 => HAND::ROCK,
        1 => HAND::PAPER,
        2 => HAND::SCISSORS,
        _ => panic!("invalid hand idx"),
    }
}

fn char_to_game(c: char) -> GAME {
    match c {
        'X' => GAME::LOSS,
        'Y' => GAME::DRAW,
        'Z' => GAME::WIN,
        _ => panic!("invalid hand char"),
    }
}

fn game(o: HAND, y: HAND) -> GAME {
    match (o, y) {
        (HAND::ROCK, HAND::SCISSORS) => GAME::LOSS,
        (HAND::SCISSORS, HAND::ROCK) => GAME::WIN,
        _ => {
            if o < y {
                GAME::WIN
            } else if o > y {
                GAME::LOSS
            } else {
                GAME::DRAW
            }
        }
    }
}

fn find_my_hand(o: HAND, g: GAME) -> HAND {
    for i in 0..=2 {
        let h = idx_to_hand(i);
        if game(o, h) == g {
            return h;
        }
    }
    panic!("failed to find matching hand")
}

fn points(opponent: char, game_char: char) -> u32 {
    let (oi, _) = RPS_ABC
        .iter()
        .enumerate()
        .find(|(_, x)| x.eq(&&opponent))
        .unwrap();
    let o = idx_to_hand(oi);
    let g = char_to_game(game_char);
    let y = find_my_hand(o, g);

    match g {
        GAME::LOSS => y as u32 + 1 + 0,
        GAME::DRAW => y as u32 + 1 + 3,
        GAME::WIN => y as u32 + 1 + 6,
    }
}

fn main() {
    let mut sum = 0;

    for line in stdin().lines().flatten() {
        if let Some(o) = line.chars().next() {
            let y = line.chars().last().unwrap();
            sum += points(o, y);
        }
    }

    println!("{}", sum)
}
