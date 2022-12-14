use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // _ = main1();
    _ = main2();
}

#[derive(Debug)]
enum Element {
    Stone,
    Sand,
}

fn parse_line(line: String, map: &mut HashMap<(u64, u64), Element>) {
    let points: Vec<(u64, u64)> = line
        .split(" -> ")
        .map(|s| {
            let v = s
                .splitn(2, ",")
                .into_iter()
                .map(|v| v.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            (v[0], v[1])
        })
        .collect();
    for point_idx in 0..points.len() - 1 {
        let first_point = points[point_idx];
        let second_point = points[point_idx + 1];
        println!("first_point: {first_point:?}, second_point: {second_point:?}");

        let xdiff = first_point.0 as i64 - second_point.0 as i64;

        if xdiff != 0 {
            let mut start = first_point;
            let mut end = second_point;
            if first_point.0 > second_point.0 {
                start = second_point;
                end = first_point;
            }
            for x in start.0..=end.0 {
                map.insert((x, start.1), Element::Stone);
            }
        } else {
            let mut start = first_point;
            let mut end = second_point;
            if first_point.1 > second_point.1 {
                start = second_point;
                end = first_point;
            }
            for y in start.1..=end.1 {
                map.insert((start.0, y), Element::Stone);
            }
        }
    }
}

fn print_map(map: &HashMap<(u64, u64), Element>) {
    let v = map.keys().next().unwrap();
    let mut min_x = v.0;
    let mut max_x = v.0;
    let mut min_y = v.1;
    let mut max_y = v.1;

    for coord in map.keys().into_iter() {
        if coord.0 < min_x {
            min_x = coord.0;
        }
        if coord.1 < min_y {
            min_y = coord.1;
        }
        if coord.0 > max_x {
            max_x = coord.0;
        }
        if coord.1 > max_y {
            max_y = coord.1;
        }
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            match map.get(&(x, y)) {
                Some(e) => match e {
                    Element::Stone => print!("#"),
                    Element::Sand => print!("o"),
                },
                None => print!("."),
            }
        }
        println!()
    }
}

fn main1() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut m = HashMap::new();
    for l in reader.lines() {
        let line = l.unwrap();
        parse_line(line, &mut m);
    }
    println!("map: {:?}", m);
    print_map(&m);

    let mut counter = 0;
    // drop sand
    'outer: loop {
        let mut sand_pos = (500, 0);
        loop {
            if sand_pos.1 > 1_000 {
                break 'outer;
            }

            let below = (sand_pos.0, sand_pos.1 + 1);
            if !(&m).contains_key(&below) {
                sand_pos = below;
                continue;
            }
            let below_left = (sand_pos.0 - 1, sand_pos.1 + 1);
            if !(&m).contains_key(&below_left) {
                sand_pos = below_left;
                continue;
            }
            let below_right = (sand_pos.0 + 1, sand_pos.1 + 1);
            if !(&m).contains_key(&below_right) {
                sand_pos = below_right;
                continue;
            }
            m.insert(sand_pos, Element::Sand);
            break;
        }
        print_map(&m);
        println!();
        counter += 1;
    }
    println!("counter: {counter}");
    Ok(())
}

fn main2() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut m = HashMap::new();
    for l in reader.lines() {
        let line = l.unwrap();
        parse_line(line, &mut m);
    }
    println!("map: {:?}", m);
    print_map(&m);

    let min_y = (&m).keys().map(|v| v.1).max().unwrap();
    let floor = min_y + 2;

    let mut counter = 0;
    // drop sand
    'outer: loop {
        let mut sand_pos = (500, 0);
        loop {
            if sand_pos.1 >= (floor - 1) {
                m.insert(sand_pos, Element::Sand);
                break;
            }
            let below = (sand_pos.0, sand_pos.1 + 1);
            if !(&m).contains_key(&below) {
                sand_pos = below;
                continue;
            }
            let below_left = (sand_pos.0 - 1, sand_pos.1 + 1);
            if !(&m).contains_key(&below_left) {
                sand_pos = below_left;
                continue;
            }
            let below_right = (sand_pos.0 + 1, sand_pos.1 + 1);
            if !(&m).contains_key(&below_right) {
                sand_pos = below_right;
                continue;
            }
            if (m.contains_key(&sand_pos)) {
                // position blocked?
                break 'outer;
            }
            m.insert(sand_pos, Element::Sand);
            break;
        }
        // print_map(&m);
        // println!();
        counter += 1;
    }
    print_map(&m);
    println!();
    println!("counter: {counter}");
    Ok(())
}
