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

    let mut matrix: Vec<Vec<i16>> = vec![];

    for l in reader.lines() {
        let line = l.unwrap();
        matrix.push(
            line.bytes()
                .map(|b| return (b as char).to_string().parse::<i16>().unwrap())
                .collect(),
        );
    }

    println!("matrix: {:?}", matrix);
    let mut invisible_counter = 0;
    for y in 1..matrix.len() - 1 {
        for x in 1..(matrix.get(0).unwrap().len() - 1) {
            let tree_height = matrix[y][x];

            let mut invisible_from_above = false;
            for yt in 0..y {
                if matrix[yt][x] >= tree_height {
                    invisible_from_above = true;
                }
            }
            let mut invisible_from_below = false;
            for yt in (y + 1)..matrix.len() {
                if matrix[yt][x] >= tree_height {
                    invisible_from_below = true;
                }
            }
            let mut invisible_from_left = false;
            for xt in 0..x {
                if matrix[y][xt] >= tree_height {
                    invisible_from_left = true;
                }
            }
            let mut invisible_from_right = false;
            for xt in (x + 1)..matrix.get(0).unwrap().len() {
                if matrix[y][xt] >= tree_height {
                    invisible_from_right = true;
                }
            }

            if invisible_from_above
                && invisible_from_below
                && invisible_from_left
                && invisible_from_right
            {
                invisible_counter += 1;
            }
        }
    }
    println!("invisible_counter: {:?}", invisible_counter);
    let size = matrix.len() * matrix.get(0).unwrap().len();
    println!("visible: {}", size - invisible_counter);

    Ok(())
}

fn main2() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<i16>> = vec![];

    for l in reader.lines() {
        let line = l.unwrap();
        matrix.push(
            line.bytes()
                .map(|b| return (b as char).to_string().parse::<i16>().unwrap())
                .collect(),
        );
    }

    println!("matrix: {:?}", matrix);
    let mut max_score = 0;
    for y in 1..matrix.len() - 1 {
        for x in 1..(matrix.get(0).unwrap().len() - 1) {
            let tree_height = matrix[y][x];

            let mut score_up = 0;
            for yt in (0..y).rev() {
                if matrix[yt][x] >= tree_height {
                    score_up += 1;
                    break;
                }
                score_up += 1;
            }
            let mut score_below = 0;
            for yt in (y + 1)..matrix.len() {
                if matrix[yt][x] >= tree_height {
                    score_below += 1;
                    break;
                }
                score_below += 1;
            }
            let mut score_left = 0;
            for xt in (0..x).rev() {
                if matrix[y][xt] >= tree_height {
                    score_left += 1;
                    break;
                }
                score_left += 1;
            }
            let mut score_right = 0;
            for xt in (x + 1)..matrix.get(0).unwrap().len() {
                if matrix[y][xt] >= tree_height {
                    score_right += 1;
                    break;
                }
                score_right += 1;
            }
            let score = score_below * score_up * score_left * score_right;
            println!("score at {}, {}: {}", y, x, score);
            if score > max_score {
                max_score = score;
            }
        }
        println!("max score: {}", max_score)
    }

    Ok(())
}
