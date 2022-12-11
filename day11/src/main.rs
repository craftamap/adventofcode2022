use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader}, collections::HashSet,
};

fn main() {
    _ = main1();
    _ = main2();
}

fn parse_monkey_lines(monkey_lines: &Vec<String>) -> Option<ParsedMonkey> {
    // first line should contain the id
    let id: u64 = monkey_lines
        .get(0)
        .unwrap()
        .strip_prefix("Monkey ")
        .unwrap()
        .strip_suffix(":")
        .unwrap()
        .parse()
        .unwrap();
    // second line should contain starting items:
    // NOTE: these are the worry levels the items start with, not the id of the item
    let items: Vec<u64> = monkey_lines
        .get(1)
        .unwrap()
        .strip_prefix("  Starting items: ")
        .unwrap()
        .split(", ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    // next line is operation
    let operation_parts: Vec<&str> = monkey_lines
        .get(2)
        .unwrap()
        .strip_prefix("  Operation: new = old ")
        .unwrap()
        .splitn(2, " ")
        .collect();
    let operation = match *operation_parts.get(0).unwrap() {
        "*" => match *operation_parts.get(1).unwrap() {
            "old" => Operation::Power,
            number @ _ => Operation::Multiplication(number.parse::<u64>().unwrap()),
        },
        "+" => match *operation_parts.get(1).unwrap() {
            number @ _ => Operation::Addition(number.parse::<u64>().unwrap()),
        },
        _ => unreachable!(),
    };
    // next operation is test
    let divisible_by: u64 = monkey_lines
        .get(3)
        .unwrap()
        .strip_prefix("  Test: divisible by ")
        .unwrap()
        .parse()
        .unwrap();

    // if true
    let true_monkey_id: u64 = monkey_lines
        .get(4)
        .unwrap()
        .strip_prefix("    If true: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap();
    // if false
    let false_monkey_id: u64 = monkey_lines
        .get(5)
        .unwrap()
        .strip_prefix("    If false: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap();

    Some(ParsedMonkey {
        id,
        start_items: items,
        operation,
        divisible_by,
        true_monkey_id,
        false_monkey_id,
    })
}

#[derive(Debug)]
enum Operation {
    Power,
    Multiplication(u64),
    Addition(u64),
}

#[derive(Debug)]
struct ParsedMonkey {
    id: u64,
    start_items: Vec<u64>,
    operation: Operation,
    divisible_by: u64,
    true_monkey_id: u64,
    false_monkey_id: u64,
}

fn main1() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut monkey_lines: Vec<Vec<String>> = vec![];
    let mut collector = vec![];

    for l in reader.lines() {
        let line = l.unwrap();
        if line == "" {
            monkey_lines.push(collector.to_owned());
            collector = vec![];
            continue;
        }
        collector.push(line);
    }
    monkey_lines.push(collector.to_owned());

    println!("{:?}", monkey_lines);

    let collected: Vec<ParsedMonkey> = monkey_lines
        .iter()
        .map(|ls| parse_monkey_lines(ls).unwrap())
        .collect();
    println!("{:?}", collected);

    let mut state: Vec<Vec<u64>> = vec![];
    let mut inspects: Vec<u64> = vec![];

    for parsed_monkey in collected.iter() {
        state.push(parsed_monkey.start_items.to_owned());
        inspects.push(0);
    }

    println!("{:?}", state);

    for i in 0..20 {
        println!("Round {i} start");
        for monkey_id in 0..state.len() {
            let monkey_state = state.get(monkey_id).unwrap().to_owned();
            let monkey_behaviour = collected.get(monkey_id).unwrap();
            for worry_level in monkey_state.to_owned().iter() {
                println!("monkey {monkey_id} inspects item with {worry_level}");
                inspects[monkey_id] = inspects.get(monkey_id).unwrap() + 1;
                let new_worry_level = match monkey_behaviour.operation {
                    Operation::Power => worry_level.pow(2),
                    Operation::Multiplication(n) => worry_level * n,
                    Operation::Addition(n) => worry_level + n,
                };
                println!(
                    "new worry level after {:?}:  {new_worry_level}",
                    monkey_behaviour.operation
                );
                let new_new_worry_level = new_worry_level / 3;
                println!("new new worry level after getting bored:  {new_new_worry_level}");
                let throw_to = if (new_new_worry_level % monkey_behaviour.divisible_by == 0) {
                    println!(
                        "{new_new_worry_level} is divisible_by {0}",
                        monkey_behaviour.divisible_by
                    );
                    monkey_behaviour.true_monkey_id as usize
                } else {
                    println!(
                        "{new_new_worry_level} is not divisible_by {0}",
                        monkey_behaviour.divisible_by
                    );
                    monkey_behaviour.false_monkey_id as usize
                };
                println!("throw item with {new_new_worry_level} to {throw_to}");
                state
                    .get_mut(throw_to as usize)
                    .unwrap()
                    .push(new_new_worry_level);
                state.get_mut(monkey_id).unwrap().remove(0);
            }
        }

        println!("Round {i} end");
        println!("{:?}", state);
    }

    inspects.sort();
    inspects.reverse();
    println!("inspects: {:?}", inspects);
    let first = inspects.get(0).unwrap();
    let second = inspects.get(1).unwrap();
    println!("level of monkey business {}", first * second);

    Ok(())
}

fn main2() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut monkey_lines: Vec<Vec<String>> = vec![];
    let mut collector = vec![];

    for l in reader.lines() {
        let line = l.unwrap();
        if line == "" {
            monkey_lines.push(collector.to_owned());
            collector = vec![];
            continue;
        }
        collector.push(line);
    }
    monkey_lines.push(collector.to_owned());

    println!("{:?}", monkey_lines);

    let collected: Vec<ParsedMonkey> = monkey_lines
        .iter()
        .map(|ls| parse_monkey_lines(ls).unwrap())
        .collect();
    println!("{:?}", collected);

    let mut state: Vec<Vec<u128>> = vec![];
    let mut inspects: Vec<u64> = vec![];

    for parsed_monkey in collected.iter() {
        state.push(
            parsed_monkey
                .start_items
                .iter()
                .map(|i| *i as u128)
                .collect(),
        );
        inspects.push(0);
    }

    let gcd = collected.iter().map(|m| m.divisible_by).collect::<HashSet<u64>>().iter().product::<u64>();
    println!("gcd {:?}", gcd);

    println!("{:?}", state);

    for i in 0..10000 {
        println!("Round {i} start");
        for monkey_id in 0..state.len() {
            let monkey_state = state.get(monkey_id).unwrap().to_owned();
            let monkey_behaviour = collected.get(monkey_id).unwrap();
            for worry_level in monkey_state.to_owned().iter() {
                // println!("monkey {monkey_id} inspects item with {worry_level}");
                inspects[monkey_id] = inspects.get(monkey_id).unwrap() + 1;
                let mut new_worry_level = match monkey_behaviour.operation {
                    Operation::Power => worry_level.pow(2),
                    Operation::Multiplication(n) => worry_level * n as u128,
                    Operation::Addition(n) => worry_level + n as u128,
                };
                // println!(
                //     "new worry level after {:?}:  {new_worry_level}",
                //     monkey_behaviour.operation
                // );
                if new_worry_level > gcd.into() {
                    new_worry_level = new_worry_level % gcd as u128;
                }
                // println!(
                //     "new worry level after optimization: {new_worry_level}",
                // );
                let throw_to = if (new_worry_level % monkey_behaviour.divisible_by as u128 == 0) {
                    // println!(
                    //     "{new_worry_level} is divisible_by {0}",
                    //     monkey_behaviour.divisible_by
                    // );
                    monkey_behaviour.true_monkey_id as usize
                } else {
                    // println!(
                    //     "{new_worry_level} is not divisible_by {0}",
                    //     monkey_behaviour.divisible_by
                    // );
                    monkey_behaviour.false_monkey_id as usize
                };
                // println!("throw item with {new_worry_level} to {throw_to}");
                state
                    .get_mut(throw_to as usize)
                    .unwrap()
                    .push(new_worry_level);
                state.get_mut(monkey_id).unwrap().remove(0);
            }
        }

        println!("Round {i} end");
        println!("{:?}", state);
    }

    inspects.sort();
    inspects.reverse();
    println!("inspects: {:?}", inspects);
    let first = inspects.get(0).unwrap();
    let second = inspects.get(1).unwrap();
    println!("level of monkey business {}", first * second);

    Ok(())
}
