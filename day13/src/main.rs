use std::{
    cmp::max,
    error::Error,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
};

use json::{array, JsonValue};

fn main() {
    // _ = main1();
    _ = main2();
}

fn parse_line(line: String) -> JsonValue {
    // FIXME: Dont use json library for parsing?
    return json::parse(&line).unwrap();
}

#[derive(Debug)]
enum Decision {
    Right,
    Wrong,
    Undecided,
}

fn compare(left: &JsonValue, right: &JsonValue) -> Decision {
    if right.is_null() {
        return Decision::Wrong;
    }
    if left.is_null() {
        return Decision::Right;
    }

    if left.is_number() && right.is_number() {
        if left.as_u64().unwrap() < right.as_u64().unwrap() {
            return Decision::Right;
        }
        if left.as_u64().unwrap() > right.as_u64().unwrap() {
            return Decision::Wrong;
        }
        return Decision::Undecided;
    }

    let l: JsonValue;
    if left.is_number() {
        l = array![left.to_owned()];
    } else {
        l = left.to_owned();
    }
    let r: JsonValue;
    if right.is_number() {
        r = array![right.to_owned()];
    } else {
        r = right.to_owned();
    }

    for i in 0..max(l.len(), r.len()) {
        let d = compare(&l[i], &r[i]);
        if !matches!(d, Decision::Undecided) {
            return d;
        }
    }
    Decision::Undecided
}

fn main1() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let mut index = 1;
    let mut sum = 0;
    while let Some(Ok(first_line)) = lines.next() {
        let second_line = lines.next().unwrap().unwrap();
        println!("{first_line} {second_line}");
        let empty_line = lines.next().unwrap_or(Ok("".to_string())).unwrap();
        if empty_line != "" {
            unreachable!()
        }

        let first = parse_line(first_line);
        let second = parse_line(second_line);
        let decision = compare(&first, &second);
        println!("{:?}", decision);

        if matches!(decision, Decision::Right) {
            sum += index;
        }

        index += 1;
    }
    println!("sum: {sum}");
    Ok(())
}

fn main2() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let lines = reader.lines();

    let mut objects = lines
        .filter(|l| l.as_ref().unwrap().to_string() != "")
        .map(|l| parse_line(l.unwrap()))
        .collect::<Vec<JsonValue>>();
    objects.push(array![array![JsonValue::Number(2.into())]]);
    objects.push(array![array![JsonValue::Number(6.into())]]);

    objects.sort_by(|v1, v2| match compare(v1, v2) {
        Decision::Right => std::cmp::Ordering::Less,
        Decision::Wrong => std::cmp::Ordering::Greater,
        Decision::Undecided => unreachable!(),
    });


    let mut positions = vec![];
    for (idx, o) in objects.iter().enumerate() {
        if (o.len() == 1
            && o[0].len() == 1
            && o[0][0].is_number()
            && (o[0][0].as_u64().unwrap() == 2 || o[0][0].as_u64().unwrap() == 6))
        {
            println!("found o {} at i: {}", o, idx+1);
            positions.push((idx+1) as u64);
        }
        println!("{}", o);
    }
    println!("decoder key: {}", positions.iter().product::<u64>());
    Ok(())
}
