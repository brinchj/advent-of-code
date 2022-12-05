use std::{io::stdin, str::Chars};

#[derive(Debug)]
struct Row {
    pub crates: Vec<Option<char>>
}

impl Row {
    fn get(cs: &mut Chars) -> Option<Option<char>> {
        match (cs.next(), cs.next(), cs.next(), cs.next()) {
            (Some('['), letter, Some(']'), _) => {
                Some(letter)
            },
            (Some(' '), Some(' '), Some(' '), _) => {
                Some(None)
            }
            _ => None,
        }
    }

    fn from_str(s: &str) -> Option<Row> {
        let mut cs = s.chars();
        let mut out = vec![];

        let mut current = Self::get(&mut cs);
        while current.is_some() {
            out.push(current.unwrap());
            current = Self::get(&mut cs);
        }
        
        return if out.is_empty() { None } else { Some(Row{crates: out}) }
    }
}

#[derive(Debug)]
struct Stacks {
    pub stacks: Vec<Vec<char>>
}

impl Stacks {
    fn from_rows(rs: Vec<Row>) -> Stacks {
        let mut stacks = vec![];
        for _ in &rs[0].crates {
            stacks.push(vec![]);
        }

        for r in &rs {
            for (idx, c) in r.crates.iter().enumerate() {
                if let Some(letter) = c {
                    stacks[idx].push(*letter)
                }
            }
        }

        for s in stacks.iter_mut() {
            s.reverse()
        }

        Stacks{stacks}
    }
}

#[derive(Debug)]
struct CommandMove {
    from: usize,
    to: usize,
    count: usize,
}

impl CommandMove {
    fn from_str(s: &str) -> Option<CommandMove> {
        let groups: Vec<&str> = s.split(' ').collect();
        if groups.len() == 6 {
            match (groups[0], groups[2], groups[4]) {
                ("move", "from", "to") => {
                    return Some(CommandMove{
                        count: str::parse(groups[1]).unwrap(),
                        from: str::parse(groups[3]).unwrap(),
                        to: str::parse(groups[5]).unwrap(),
                    })
                }
                _ => return None
            }
        }
        return None
    }
}

fn main() {
    let mut rows = vec![];
    for lines_res in stdin().lines() {
        if let Ok(line) = lines_res {
            if let Some(row) = Row::from_str(line.as_str()) {
                rows.push(row)
            } else {
                break
            }
        }
    }

    let mut stacks = Stacks::from_rows(rows);

    for lines_res in stdin().lines() {
        if let Ok(line) = lines_res {
            if let Some(cmd) = CommandMove::from_str(&line) {
                let l = stacks.stacks[cmd.from - 1].len();
                let moved: Vec<char> = stacks.stacks[cmd.from - 1].drain(l-cmd.count..).collect();
                stacks.stacks[cmd.to - 1].extend(moved);
            }
        }
    }
    
    for s in stacks.stacks {
        print!("{}", s.iter().last().unwrap())
    }
    println!();
}
