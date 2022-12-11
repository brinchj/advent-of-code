use std::{
    collections::HashMap,
    io::{stdin, Read},
    mem,
};

use nom::{
    bytes::complete::{tag, take_while, take_while_m_n},
    character,
    character::is_alphabetic,
    IResult, ParseTo,
};

use nom;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Square,
    Plus(u64),
    Mult(u64),
}

#[derive(Debug, Clone, Copy)]
enum Test {
    DivisibleBy(u64),
}

type MonkeyId = u64;

#[derive(Debug, Clone)]
struct Monkey {
    idx: MonkeyId,
    items: Vec<u64>,
    operation: Operation,
    test: Test,
    if_true: MonkeyId,
    if_false: MonkeyId,
    inspected_items: usize,
}

fn parse_starting_items(input: &[u8]) -> IResult<&[u8], Vec<u64>> {
    let (input, _) = tag("Starting items: ")(input)?;
    let (input, items) = nom::multi::separated_list0(tag(", "), character::complete::u64)(input)?;
    Ok((input, items))
}

fn parse_operation(input: &[u8]) -> IResult<&[u8], Operation> {
    let (input, _) = tag("Operation: new = old ")(input)?;
    let (input, op) = nom::character::complete::one_of("*+")(input)?;
    let (input, _) = tag(" ")(input)?;

    let try_old = tag("old")(input);
    let try_num = character::complete::u64(input);

    match (op, try_old, try_num) {
        ('*', Ok((input, _)), Err(_)) => Ok((input, Operation::Square)),
        ('+', Ok(_), Err(_)) => panic!("old + old"),
        ('*', Err(_), Ok((input, n))) => Ok((input, Operation::Mult(n))),
        ('+', Err(_), Ok((input, n))) => Ok((input, Operation::Plus(n))),
        (_, Err(err), _) => IResult::Err(err),
        (_, _, Err(err)) => IResult::Err(err),
        (_, _, _) => unreachable!(),
    }
}

fn parse_test(input: &[u8]) -> IResult<&[u8], Test> {
    let (input, _) = tag("Test: divisible by ")(input)?;
    let (input, n) = character::complete::u64(input)?;
    Ok((input, Test::DivisibleBy(n)))
}

fn parse_result<'i>(input: &'i [u8], name: &str) -> IResult<&'i [u8], MonkeyId> {
    let (input, _) = tag(format!("If {}: throw to monkey ", name).as_bytes())(input)?;
    character::complete::u64(input)
}

fn parse_monkey(input: &[u8]) -> IResult<&[u8], Monkey> {
    let (input, _) = take_while(|c| !is_alphabetic(c))(input)?;

    let (input, _) = tag("Monkey ")(input)?;
    let (input, monkey_id) = character::complete::u64(input)?;

    let (input, _) = take_while(|c| !is_alphabetic(c))(input)?;
    let (input, items) = parse_starting_items(input)?;

    let (input, _) = take_while(|c| !is_alphabetic(c))(input)?;
    let (input, operation) = parse_operation(input)?;

    let (input, _) = take_while(|c| !is_alphabetic(c))(input)?;
    let (input, test) = parse_test(input)?;

    let (input, _) = take_while(|c| !is_alphabetic(c))(input)?;
    let (input, if_true) = parse_result(input, "true")?;

    let (input, _) = take_while(|c| !is_alphabetic(c))(input)?;
    let (input, if_false) = parse_result(input, "false")?;

    Ok((
        input,
        Monkey {
            idx: monkey_id,
            items,
            operation,
            test,
            if_true,
            if_false,
            inspected_items: 0,
        },
    ))
}

fn parse_monkeys(input: &[u8]) -> IResult<&[u8], Vec<Monkey>> {
    nom::multi::many0(parse_monkey)(input)
}

fn exec_operation(op: Operation, item: u64, modolu: u64) -> u64 {
    match op {
        Operation::Square => (item * item) % modolu,
        Operation::Plus(n) => (item + n) % modolu,
        Operation::Mult(n) => (item * n) % modolu,
    }
}

fn test_condition(n: u64, test: Test, if_true: MonkeyId, if_false: MonkeyId) -> MonkeyId {
    let b = match test {
        Test::DivisibleBy(m) => (n as u64) % m == 0,
    };

    if b {
        if_true
    } else {
        if_false
    }
}

fn main() {
    let mut buf = vec![];
    stdin().lock().read_to_end(&mut buf).unwrap();

    let (_, monkeys) = parse_monkeys(&buf).unwrap();
    let mut mm: HashMap<u64, Monkey> = monkeys.into_iter().map(|m| (m.idx, m)).collect();

    let mut ids: Vec<u64> = mm.keys().copied().collect();
    ids.sort();

    let mut modulo = 1u64;
    for m in mm.values() {
        match m.test {
            Test::DivisibleBy(m) => modulo *= m,
        }
    }

    for _ in 0..10000 {
        for id in &ids {
            let m = mm[id].clone();

            for item in m.items {
                let new_item = exec_operation(m.operation, item, modulo);
                let next = test_condition(new_item, m.test, m.if_true, m.if_false);

                mm.get_mut(id).unwrap().inspected_items += 1;
                mm.get_mut(&next).unwrap().items.push(new_item);
            }

            mm.get_mut(id).unwrap().items = vec![];
        }
    }

    for i in ids {
        println!("{}: [{}] {:?}", i, mm[&i].inspected_items, mm[&i].items)
    }

    let mut v: Vec<usize> = mm.values().map(|m| m.inspected_items).collect();
    v.sort();
    v.reverse();
    println!("{}", v[0] * v[1]);
}
