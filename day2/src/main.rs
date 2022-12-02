use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Res {
    Win,
    Tie,
    Loose,
}

fn parse_move1(mov: char) -> Move {
    return match mov {
        'A' | 'X' => Move::Rock,
        'B' | 'Y' => Move::Paper,
        'C' | 'Z' => Move::Scissors,
        _ => unreachable!("wuppsi"),
    };
}

fn main1() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut score = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        let other = l.chars().nth(0).unwrap();
        let mine = l.chars().nth(2).unwrap();
        let other_move = parse_move1(other);
        let my_move = parse_move1(mine);

        let r = match my_move {
            Move::Rock => {
                score += 1;
                match other_move {
                    Move::Rock => Res::Tie,
                    Move::Paper => Res::Loose,
                    Move::Scissors => Res::Win,
                }
            }
            Move::Paper => {
                score += 2;
                match other_move {
                    Move::Rock => Res::Win,
                    Move::Paper => Res::Tie,
                    Move::Scissors => Res::Loose,
                }
            }
            Move::Scissors => {
                score += 3;
                match other_move {
                    Move::Rock => Res::Loose,
                    Move::Paper => Res::Win,
                    Move::Scissors => Res::Tie,
                }
            }
        };

        match r {
            Res::Win => score += 6,
            Res::Tie => score += 3,
            Res::Loose => score += 0,
        }
    }

    println!("score {}", score);

    Ok(())
}

fn parse_move2(mov: char) -> Move {
    return match mov {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissors,
        _ => unreachable!("wuppsi"),
    };
}

fn parse_res2(mov: char) -> Res {
    return match mov {
        'X' => Res::Loose,
        'Y' => Res::Tie,
        'Z' => Res::Win,
        _ => unreachable!("wuppsi"),
    };
}
fn main2() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut score = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        let other = l.chars().nth(0).unwrap();
        let mine = l.chars().nth(2).unwrap();
        let other_move = parse_move2(other);
        let my_res = parse_res2(mine);

        let r = match my_res {
            Res::Win => {
                score += 6;
                match other_move {
                    Move::Rock => Move::Paper,
                    Move::Paper => Move::Scissors,
                    Move::Scissors => Move::Rock,
                }
            }
            Res::Tie => {
                score += 3;
                match other_move {
                    Move::Rock => Move::Rock,
                    Move::Paper => Move::Paper,
                    Move::Scissors => Move::Scissors,
                }
            }
            Res::Loose => {
                score += 0;
                match other_move {
                    Move::Rock => Move::Scissors,
                    Move::Paper => Move::Rock,
                    Move::Scissors => Move::Paper,
                }
            }
        };

        match r {
            Move::Rock => score += 1,
            Move::Paper => score += 2,
            Move::Scissors => score += 3,
        }
    }

    println!("score {}", score);

    Ok(())
}

fn main() {
    main2().unwrap();
}
