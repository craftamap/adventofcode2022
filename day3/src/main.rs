use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    //return main1();
    return main2();
}
fn main2() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    let collected: Vec<_> = reader.lines().collect();

    for chunk in collected.chunks(3) {
        let mut first = HashSet::new();
        let l = chunk[0].as_ref().unwrap();
        for idx in 0..l.len() {
            first.insert(l.chars().nth(idx).unwrap());
        }

        let mut second = HashSet::new();
        let l = chunk[1].as_ref().unwrap();
        for idx in 0..l.len() {
            let char = l.chars().nth(idx).unwrap();
            if first.contains(&char) {
                second.insert(char);
            }
        }

        println!("second: {:?}", second);

        let l = chunk[2].as_ref().unwrap();
        let mut common = None;
        for idx in 0..l.len() {
            let char = l.chars().nth(idx).unwrap();
            if second.contains(&char) {
                common = Some(char);
                break;
            }
        }

        let priority = match common.unwrap() {
            char @ 'a'..='z' => char as u32 - ('a' as u32 - 1),
            char @ 'A'..='Z' => char as u32 - ('A' as u32 - 1 - 26),
            _ => unreachable!(),
        };
        println!("char: {}, priority: {}", priority, priority);
        sum += priority;
    }

    println!("sum {}", sum);

    Ok(())
}

fn main1() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    for l in reader.lines() {
        let line = l.unwrap();
        let compartment_size = line.len() / 2;
        let mut left_items = HashSet::new();
        for idx in 0..compartment_size {
            left_items.insert(line.chars().nth(idx).unwrap());
        }
        for idx in compartment_size..line.len() {
            let char = &line.chars().nth(idx).unwrap();
            if left_items.contains(char) {
                let priority = match char {
                    'a'..='z' => *char as u32 - ('a' as u32 - 1),
                    'A'..='Z' => *char as u32 - ('A' as u32 - 1 - 26),
                    _ => unreachable!(),
                };
                println!("char: {}, priority: {}", char, priority);
                sum += priority;
                break;
            }
        }
    }

    println!("sum {}", sum);

    Ok(())
}
