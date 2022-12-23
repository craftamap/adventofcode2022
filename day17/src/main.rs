use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // _ = main1();
    _ = main2();
}

#[derive(Clone, Debug)]
enum Item {
    Air,
    Stone,
}

fn shape_height(shape: &Vec<(isize, isize)>) -> isize {
    let mut max_y = 0;
    for i in shape.iter() {
        if i.1 > max_y {
            max_y = i.1
        }
    }
    return max_y + 1;
}

fn grid_height(grid: &Vec<Vec<Item>>) -> isize {
    for (i, row) in grid.iter().enumerate().rev() {
        for ri in row.iter() {
            if matches!(ri, Item::Stone) {
                return i as isize + 1;
            }
        }
    }

    return 0;
}

fn main1() -> Result<(), Box<dyn Error>> {
    let shape_minus: Vec<(isize, isize)> = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
    let shape_plus: Vec<(isize, isize)> = vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)];
    let shape_l: Vec<(isize, isize)> = vec![(2, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
    let shape_i: Vec<(isize, isize)> = vec![(0, 0), (0, 1), (0, 2), (0, 3)];
    let shape_square: Vec<(isize, isize)> = vec![(0, 0), (1, 0), (0, 1), (1, 1)];

    let shapes = vec![shape_minus, shape_plus, shape_l, shape_i, shape_square];

    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let line = reader.lines().next().unwrap().unwrap();
    let jet_pattern: Vec<char> = line.chars().collect();
    let air = vec![
        Item::Air,
        Item::Air,
        Item::Air,
        Item::Air,
        Item::Air,
        Item::Air,
        Item::Air,
    ];
    let mut grid = vec![air.to_vec(), air.to_vec(), air.to_vec()];
    let mut jet_index = 0;

    for i in 0..2022 {
        if i % 100 == 0 {
            println!("{}", i);
            let height = grid_height(&grid);
            println!("grid_height: {}", height);
        }
        let shape = &shapes[i % shapes.len()];
        let sh = shape_height(&shape);
        let gh = grid_height(&grid);
        if (grid.len() as isize - sh - 3) < gh {
            let diff = gh - (grid.len() as isize - sh - 3);
            for _ in 0..diff {
                grid.push(air.to_vec());
            }
        }
        grid.push(air.to_vec());

        let mut shape_position = (2, gh + (sh - 1) + 3);

        loop {
            // move according to jet
            let symbol = jet_pattern[jet_index % jet_pattern.len()];
            let new_position = (
                shape_position.0 + if symbol == '>' { 1 } else { -1 },
                shape_position.1,
            );
            let mut should_update = true;
            for part in shape.iter() {
                let part_position = (new_position.0 + part.0, new_position.1 - part.1);
                if part_position.0 < 0 || part_position.0 >= grid[0].len() as isize {
                    should_update = false;
                    break;
                }
                if matches!(
                    grid[part_position.1 as usize][part_position.0 as usize],
                    Item::Stone
                ) {
                    should_update = false;
                    break;
                }
            }
            if should_update {
                shape_position = new_position;
            }
            jet_index += 1;

            // move down
            let new_position = (shape_position.0, shape_position.1 - 1);
            let mut should_continue = true;
            for part in shape.iter() {
                let part_position = (new_position.0 + part.0, new_position.1 - part.1);
                if part_position.1 < 0 {
                    // collided with floor
                    should_continue = false;
                    break;
                }
                if (matches!(
                    grid[part_position.1 as usize][part_position.0 as usize],
                    Item::Stone
                )) {
                    should_continue = false;
                    break;
                }
            }
            if should_continue {
                shape_position = new_position;
            } else {
                // commit
                for part in shape.iter() {
                    let part_position = (shape_position.0 + part.0, shape_position.1 - part.1);
                    grid[part_position.1 as usize][part_position.0 as usize] = Item::Stone;
                }

                break;
            }
        }
    }
    // print_grid(&grid);
    let height = grid_height(&grid);
    println!("grid_height: {}", height);

    Ok(())
}

fn main2() -> Result<(), Box<dyn Error>> {
    let shape_minus: Vec<(isize, isize)> = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
    let shape_plus: Vec<(isize, isize)> = vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)];
    let shape_l: Vec<(isize, isize)> = vec![(2, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
    let shape_i: Vec<(isize, isize)> = vec![(0, 0), (0, 1), (0, 2), (0, 3)];
    let shape_square: Vec<(isize, isize)> = vec![(0, 0), (1, 0), (0, 1), (1, 1)];

    let shapes = vec![shape_minus, shape_plus, shape_l, shape_i, shape_square];

    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let line = reader.lines().next().unwrap().unwrap();
    let jet_pattern: Vec<char> = line.chars().collect();
    let air = vec![
        Item::Air,
        Item::Air,
        Item::Air,
        Item::Air,
        Item::Air,
        Item::Air,
        Item::Air,
    ];
    let mut grid = vec![air.to_vec(), air.to_vec(), air.to_vec()];
    let mut jet_index = 0;

    let mut known_combination: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let max = 1_000_000_000_000;

    for i in 0..max {
        let gh = grid_height(&grid);
        // this is adjusted from reddit: https://github.com/ritesh-singh/aoc-2022-kotlin/blob/main/src/day17/Day17.kt#L105
        // don't remember before 500 to ignore beginning - we assume that it gets periodic at some
        // point
        if i > 500 {
            if !known_combination
                .contains_key(&(jet_index % jet_pattern.len(), i as usize % shapes.len()))
            {
                known_combination.insert(
                    (jet_index % jet_pattern.len(), i as usize % shapes.len()),
                    (i, gh as usize),
                );
            } else {
                println!("match {i}");
                let (known_i, known_gh) = known_combination
                    .get(&(jet_index % jet_pattern.len(), i as usize % shapes.len()))
                    .unwrap();
                let period = i - known_i;
                println!("period: {period}");
                // period that fits with the goal
                if i % period == max % period {
                    println!("match 2 {i}");
                    let height_cycle = gh - known_gh.to_owned() as isize;
                    let remaining_rocks = max - i;
                    let cycles_remaining = (remaining_rocks / period) + 1;
                    println!(
                        "{}",
                        known_gh.to_owned() as isize
                            + (height_cycle * cycles_remaining.to_owned() as isize)
                    );
                    break;
                }
            }
        }
        let shape = &shapes[i as usize % shapes.len()];
        let sh = shape_height(&shape);
        if (grid.len() as isize - sh - 3) < gh {
            let diff = gh - (grid.len() as isize - sh - 3);
            for _ in 0..diff {
                grid.push(air.to_vec());
            }
        }
        grid.push(air.to_vec());

        let mut shape_position = (2, gh + (sh - 1) + 3);

        loop {
            // move according to jet
            let symbol = jet_pattern[jet_index % jet_pattern.len()];
            let new_position = (
                shape_position.0 + if symbol == '>' { 1 } else { -1 },
                shape_position.1,
            );
            let mut should_update = true;
            for part in shape.iter() {
                let part_position = (new_position.0 + part.0, new_position.1 - part.1);
                if part_position.0 < 0 || part_position.0 >= grid[0].len() as isize {
                    should_update = false;
                    break;
                }
                if matches!(
                    grid[part_position.1 as usize][part_position.0 as usize],
                    Item::Stone
                ) {
                    should_update = false;
                    break;
                }
            }
            if should_update {
                shape_position = new_position;
            }
            jet_index += 1;

            // move down
            let new_position = (shape_position.0, shape_position.1 - 1);
            let mut should_continue = true;
            for part in shape.iter() {
                let part_position = (new_position.0 + part.0, new_position.1 - part.1);
                if part_position.1 < 0 {
                    // collided with floor
                    should_continue = false;
                    break;
                }
                if (matches!(
                    grid[part_position.1 as usize][part_position.0 as usize],
                    Item::Stone
                )) {
                    should_continue = false;
                    break;
                }
            }
            if should_continue {
                shape_position = new_position;
            } else {
                // commit
                for part in shape.iter() {
                    let part_position = (shape_position.0 + part.0, shape_position.1 - part.1);
                    grid[part_position.1 as usize][part_position.0 as usize] = Item::Stone;
                }

                break;
            }
        }
    }
    // print_grid(&grid);
    let height = grid_height(&grid);
    println!("grid_height: {}", height);

    Ok(())
}

fn print_grid(grid: &Vec<Vec<Item>>) {
    for row in grid.iter().rev() {
        for item in row {
            print!(
                "{}",
                if matches!(item, Item::Stone) {
                    "#"
                } else {
                    "."
                }
            )
        }
        println!()
    }
}
