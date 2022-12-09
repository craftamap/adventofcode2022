use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // _ = main1();
    _ = main2();
}

#[derive(Debug)]
enum Motion {
    Left,
    Right,
    Up,
    Down,
}

fn parse_motion(line: String) -> Option<(Motion, u64)> {
    let mut splitted = line.splitn(2, " ");
    let m = splitted.next().unwrap();
    let n: u64 = splitted.next().unwrap().parse().unwrap();
    let r = match m {
        "L" => Some(Motion::Left),
        "R" => Some(Motion::Right),
        "U" => Some(Motion::Up),
        "D" => Some(Motion::Down),
        _ => None,
    }
    .unwrap();
    Some((r, n))
}

fn vec_between(vec1: (u64, u64), vec2: (u64, u64)) -> (i128, i128) {
    (
        vec2.0 as i128 - vec1.0 as i128,
        vec2.1 as i128 - vec1.1 as i128,
    )
}

fn vec_len(vec: (i128, i128)) -> f64 {
    (((vec.0.pow(2)) + (vec.1.pow(2))) as f64).sqrt()
}

fn vec_norm(vec: (i128, i128)) -> (i128, i128) {
    (
        vec.0 / if vec.0 == 0 { 1 } else { vec.0.abs() },
        vec.1 / if vec.1 == 0 { 1 } else { vec.1.abs() },
    )
}

fn main1() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    // x,y
    let mut head = (1000, 1000);
    let mut tail = (1000, 1000);

    let mut set: HashSet<(u64, u64)> = HashSet::new();

    for l in reader.lines() {
        let (motion, n) = parse_motion(l.unwrap()).unwrap();

        for _i in 0..n {
            println!("{:?}", motion);
            match motion {
                Motion::Left => head = (head.0 - 1, head.1),
                Motion::Right => head = (head.0 + 1, head.1),
                Motion::Up => head = (head.0, head.1 + 1),
                Motion::Down => head = (head.0, head.1 - 1),
            }

            let betw = vec_between(tail, head);
            let len = vec_len(betw);
            // no move needed if len < 2, as its next to the head
            if len >= 2.0 {
                let norm = vec_norm(betw);
                println!("vec_norm {:?}", norm);
                tail = (
                    (tail.0 as i128 + norm.0) as u64,
                    (tail.1 as i128 + norm.1) as u64,
                );
            }
            set.insert(tail);
            println!("head: {:?}", head);
            println!("tail: {:?}", tail);
            println!("");
        }
    }

    println!("set {}", set.len());

    Ok(())
}

fn main2() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    // x,y
    let mut chain: Vec<(u64, u64)> = vec![];
    for _i in 0..10 {
        chain.push((1000, 1000));
    }

    println!("chain: {:?}", chain);

    let mut set: HashSet<(u64, u64)> = HashSet::new();

    for l in reader.lines() {
        let (motion, n) = parse_motion(l.unwrap()).unwrap();

        for _i in 0..n {
            let head = chain.get_mut(0).unwrap();
            println!("{:?}", motion);
            match motion {
                Motion::Left => *head = (head.0 - 1, head.1),
                Motion::Right => *head = (head.0 + 1, head.1),
                Motion::Up => *head = (head.0, head.1 + 1),
                Motion::Down => *head = (head.0, head.1 - 1),
            }

            for i in 1..chain.len() {
                let next = chain.get(i - 1).unwrap();
                let mut current = *chain.get(i).unwrap();
                let betw = vec_between(current, *next);
                let len = vec_len(betw);
                // no move needed if len < 2, as its next to the head
                if len >= 2.0 {
                    let norm = vec_norm(betw);
                    println!("vec_norm {:?}", norm);
                    current = (
                        (current.0 as i128 + norm.0) as u64,
                        (current.1 as i128 + norm.1) as u64,
                    );
                }
                chain[i] = current;
                if i == chain.len() - 1 {
                    set.insert(current);
                };
            }
        }
    }

    println!("set {:?}", set);
    println!("set {}", set.len());
    println!("chain: {:?}", chain);

    Ok(())
}
