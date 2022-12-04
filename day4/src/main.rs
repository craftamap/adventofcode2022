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

    let mut count = 0;

    for l in reader.lines() {
        let line = l.unwrap();
        let (lhs_text, rhs_text) = line.split_once(",").unwrap();
        let (lhs_start_text, lhs_end_text) = lhs_text.split_once("-").unwrap();
        let (rhs_start_text, rhs_end_text) = rhs_text.split_once("-").unwrap();
        let lhs_start: u32 = lhs_start_text.parse().unwrap();
        let lhs_end: u32 = lhs_end_text.parse().unwrap();
        let rhs_start: u32 = rhs_start_text.parse().unwrap();
        let rhs_end: u32 = rhs_end_text.parse().unwrap();

        let lhs = (lhs_start, lhs_end);
        let rhs = (rhs_start, rhs_end);

        let earlier: &(u32, u32);
        let later: &(u32, u32);
        if lhs.0 < rhs.0 {
            earlier = &lhs;
            later = &rhs;
        } else if lhs.0 > rhs.0 {
            earlier = &rhs;
            later = &lhs;
        } else {
            if lhs.1 > rhs.1 {
                earlier = &lhs;
                later = &rhs;
            } else {
                earlier = &rhs;
                later = &lhs;
            }
        }

        // - Check if later is fully in earlier
        //   - Check if the first value of of later is in boundaries of earlier
        if !(earlier.0 <= later.0 && later.0 <= earlier.1) {
            continue;
        }
        //   - Check if the second value of of later is in boundaries of earlier
        if !(earlier.0 <= later.1 && later.1 <= earlier.1) {
            continue;
        }

        println!(
            "Match! {}, earlier: {:?}, later: {:?}",
            line, earlier, later
        );
        count += 1;
    }

    println!("total matches: {}", count);

    Ok(())
}

fn main2() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut count = 0;

    for l in reader.lines() {
        let line = l.unwrap();
        let (lhs_text, rhs_text) = line.split_once(",").unwrap();
        let (lhs_start_text, lhs_end_text) = lhs_text.split_once("-").unwrap();
        let (rhs_start_text, rhs_end_text) = rhs_text.split_once("-").unwrap();
        let lhs_start: u32 = lhs_start_text.parse().unwrap();
        let lhs_end: u32 = lhs_end_text.parse().unwrap();
        let rhs_start: u32 = rhs_start_text.parse().unwrap();
        let rhs_end: u32 = rhs_end_text.parse().unwrap();

        let lhs = (lhs_start, lhs_end);
        let rhs = (rhs_start, rhs_end);

        let earlier: &(u32, u32);
        let later: &(u32, u32);
        if lhs.0 < rhs.0 {
            earlier = &lhs;
            later = &rhs;
        } else {
            earlier = &rhs;
            later = &lhs;
        }

        if !(later.0 <= earlier.1) {
            continue;
        }

        println!(
            "Match! {}, earlier: {:?}, later: {:?}",
            line, earlier, later
        );
        count += 1;
    }

    println!("total matches: {}", count);

    Ok(())
}
