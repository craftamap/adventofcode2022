use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    _ = main1();
    _ = main2();
}

fn main1() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let line = reader.lines().next().unwrap().unwrap();
    let c = line.chars();
    let chars: Vec<char> = c.collect();
    let mut start = 0;
    let mut end = 4;
    while end <= chars.len() {
        let v = &mut chars[start..end].to_owned();
        v.sort();
        v.dedup();
        if v.len() == 4 {
            println!("first marker after {}", end);
            break;
        }

        start+=1;
        end+=1;
    }

    Ok(())
}

fn main2() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let line = reader.lines().next().unwrap().unwrap();
    let c = line.chars();
    let chars: Vec<char> = c.collect();
    let mut start = 0;
    let mut end = 14;
    while start <= chars.len() {
        let v = &mut chars[start..(if end > chars.len() {chars.len()} else {end})].to_owned();
        v.sort();
        v.dedup();
        if v.len() == 14 {
            println!("first marker after {}", end);
            break;
        }

        start+=1;
        end+=1;
    }

    Ok(())
}
