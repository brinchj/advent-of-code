use std::{cmp::Ordering, io::stdin};

use nom::{bytes::streaming::tag, multi::separated_list0, IResult};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Item {
    Number(i32),
    List(Vec<Item>),
}

fn parse(input: &[u8]) -> IResult<&[u8], Item> {
    if !input.is_empty() && nom::character::is_digit(input[0]) {
        let (input, n) = nom::character::complete::i32(input)?;
        Ok((input, Item::Number(n)))
    } else {
        let (input, _) = tag("[")(input)?;
        let (input, out) = separated_list0(tag(","), parse)(input)?;
        let (input, _) = tag("]")(input)?;
        Ok((input, Item::List(out)))
    }
}

fn compare(a: &Item, b: &Item) -> Ordering {
    match (a, b) {
        (Item::Number(n), Item::Number(m)) => i32::cmp(n, m),
        (Item::List(_), Item::Number(m)) => compare(a, &Item::List(vec![Item::Number(*m)])),
        (Item::Number(n), Item::List(_)) => compare(&Item::List(vec![Item::Number(*n)]), b),
        (Item::List(x), Item::List(y)) => {
            let res = x
                .iter()
                .zip(y.iter())
                .map(|(a, b)| compare(a, b))
                .find(|c| c != &Ordering::Equal);
            res.unwrap_or_else(|| usize::cmp(&x.len(), &y.len()))
        }
    }
}

fn main() {
    let mut items: Vec<Item> = stdin()
        .lines()
        .flatten()
        .flat_map(|l| parse(l.as_bytes()).map(|(_, a)| a).ok())
        .collect();

    let start = Item::List(vec![Item::List(vec![Item::Number(2)])]);
    let end = Item::List(vec![Item::List(vec![Item::Number(6)])]);

    items.push(start.clone());
    items.push(end.clone());
    items.sort_by(compare);

    let res = items
        .iter()
        .enumerate()
        .filter(|(_, item)| [&start, &end].contains(item))
        .map(|(idx, _)| idx + 1)
        .reduce(|a, b| a * b);

    println!("{}", res.unwrap());
}
