use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() {
    _ = main1();
    _ = main2();
}

fn parse_inital_stack(lines: Vec<String>) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let last_line = lines.last().unwrap();
    let re = Regex::new(r" (\d+)").unwrap();
    let cap = re.captures_iter(last_line);
    let last_cap = cap.last().unwrap();
    let amount_of_stacks = last_cap
        .get(0)
        .unwrap()
        .as_str()
        .trim()
        .parse::<u32>()
        .unwrap();

    let mut stacks: Vec<Vec<String>> = Vec::new();
    for _i in 0..amount_of_stacks {
        stacks.push(vec![])
    }

    let mut iter = lines.iter().rev();
    iter.next();
    'line: for line in iter {
        let mut line_chars: Vec<char> = line.chars().collect();
        println!("line_chars: {:?}", line_chars);
        let mut cursor = 0;
        for i in 0..amount_of_stacks {
            let char_at = *line_chars.get(cursor).unwrap_or(&'\n');
            if char_at == '[' {
                cursor += 1;
                let char = line_chars.get(cursor).unwrap().to_string();
                println!("char {} at {}", char, cursor);
                stacks[i as usize].push(char);
                cursor += 1;
                cursor += 1;
                cursor += 1;
            } else if char_at == ' ' {
                println!("empty, {}", cursor);
                cursor += 1;
                cursor += 1;
                cursor += 1;
                cursor += 1;
            } else {
                println!("else: {}", line_chars.get(cursor).unwrap());
                continue 'line;
            }
        }
    }

    println!("{:?}", stacks);

    Ok(stacks)
}

fn parse_task_line(line: String) -> (usize, usize, usize) {
    let splits = line.split(" ").collect::<Vec<&str>>();
    return (
        splits[1].parse().unwrap(),
        splits[3].parse::<usize>().unwrap() - 1,
        splits[5].parse::<usize>().unwrap() - 1,
    );
}

fn main1() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut initial_stack_lines: Vec<String> = Vec::new();
    let mut tasks: Vec<String> = Vec::new();

    let mut found_empty = false;
    for l in reader.lines() {
        let line = l.unwrap();
        if line == "" {
            found_empty = true;
            continue;
        }
        if !found_empty {
            initial_stack_lines.push(line);
        } else {
            tasks.push(line)
        }
    }
    let mut inital_stack = parse_inital_stack(initial_stack_lines).unwrap();
    for task in tasks {
        let (amount, from, to) = parse_task_line(task);
        for _i in 0..amount {
            let from_vec = inital_stack.get_mut(from).unwrap();
            let a = from_vec.pop().unwrap();
            let to_vec = inital_stack.get_mut(to).unwrap();
            to_vec.push(a);
            println!("{:?}", inital_stack)
        }
    }

    for st in inital_stack {
        print!("{}", st.last().unwrap());
    }
    println!();

    Ok(())
}

fn main2() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut initial_stack_lines: Vec<String> = Vec::new();
    let mut tasks: Vec<String> = Vec::new();

    let mut found_empty = false;
    for l in reader.lines() {
        let line = l.unwrap();
        if line == "" {
            found_empty = true;
            continue;
        }
        if !found_empty {
            initial_stack_lines.push(line);
        } else {
            tasks.push(line)
        }
    }
    let mut inital_stack = parse_inital_stack(initial_stack_lines).unwrap();
    for task in tasks {
        let (amount, from, to) = parse_task_line(task);

        let from_vec = inital_stack.get_mut(from).unwrap();
        let mut tmp = vec![];
        for _i in 0..amount {
            tmp.push(from_vec.pop().unwrap());
        }
        let to_vec = inital_stack.get_mut(to).unwrap();
        while let Some(a) = tmp.pop() {
            to_vec.push(a);
        }
        println!("{:?}", inital_stack)
    }

    for st in inital_stack {
        print!("{}", st.last().unwrap());
    }
    println!();

    Ok(())
}
