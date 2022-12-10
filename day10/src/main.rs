use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    _ = main1();
    _ = main2();
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i64),
}

fn parse_instruction(line: String) -> Option<Instruction> {
    if line.starts_with("noop") {
        Some(Instruction::Noop)
    } else if line.starts_with("addx") {
        let mut it = line.splitn(2, " ");
        it.next();
        let number = it.next().unwrap().parse().unwrap();
        Some(Instruction::Addx(number))
    } else {
        None
    }
}

fn main2() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut cycle: i64 = 1;

    let mut register_x: i64 = 1;

    let screen_on_clock = |cycle, register_x: i64| {
        // println!("sprite_positon: {} - {}", register_x, register_x + 2)
        if cycle % 40 == 1 {
            println!()
        }
        print!(
            "{}",
            if (register_x..register_x + 3).contains(&(cycle % 40)) {
                '#'
            } else {
                '.'
            }
        )
    };

    for l in reader.lines() {
        let instruction = parse_instruction(l.unwrap()).unwrap();
        // println!("{:?}", instruction);

        match instruction {
            Instruction::Noop => {
                screen_on_clock(cycle, register_x);
                cycle += 1;
            }
            Instruction::Addx(n) => {
                screen_on_clock(cycle, register_x);
                cycle += 1;

                screen_on_clock(cycle, register_x);
                cycle += 1;
                register_x += n;
            }
        }
    }

    Ok(())
}

fn main1() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut cycle = 1;

    let mut register_x = 1;

    let mut signals = vec![];

    let mut check_cycle = |cycle, register_x| {
        println!("cycle: {}, register_x: {}", cycle, register_x);
        if (cycle - 20) % 40 == 0 {
            println!("push {}", cycle * register_x);
            signals.push(cycle * register_x);
        }
    };

    for l in reader.lines() {
        let instruction = parse_instruction(l.unwrap()).unwrap();
        println!("{:?}", instruction);

        match instruction {
            Instruction::Noop => {
                check_cycle(cycle, register_x);
                cycle += 1;
            }
            Instruction::Addx(n) => {
                check_cycle(cycle, register_x);
                cycle += 1;

                check_cycle(cycle, register_x);
                cycle += 1;
                register_x += n;
            }
        }
    }

    println!("signals: {:?}", signals);
    println!("signals: {:?}", signals.iter().sum::<i64>());
    Ok(())
}
