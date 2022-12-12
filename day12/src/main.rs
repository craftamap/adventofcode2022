use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    _ = main1();
    // _ = main2();
}

fn main1() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut height_map = vec![];

    for l in reader.lines() {
        let mut height_line = vec![];
        let line = l.unwrap();
        for c in line.chars().into_iter() {
            height_line.push((c as i16) - 'a' as i16)
        }
        height_map.push(height_line);
    }

    println!("{:?}", height_map);

    let mut start_pos = None;
    let mut end_pos = None;
    for (y, row) in height_map.iter().enumerate() {
        for (x, v) in row.iter().enumerate() {
            if *v == -14 {
                start_pos = Some((x, y))
            }
            if *v == -28 {
                end_pos = Some((x, y))
            }
        }
    }

    fn traverse(
        height_map: Vec<Vec<i16>>,
        end_pos: (usize, usize),
        current_pos: (usize, usize),
        path: Vec<(usize, usize)>,
        cache: &mut HashMap<(usize, usize), u64>,
    ) -> u64 {
        let mut current_height = height_map[current_pos.1][current_pos.0];
        // start_pos is height: 0
        if current_height == -14 {
            current_height = 0;
        }
        if current_pos == end_pos {
            println!("found solution with path {:?}", path);
            for i in path.iter() {
                print!("{} ", height_map[i.1][i.0]);
            }
            println!("length of solution: {}", path.len());
            return (path.len()) as u64;
        }
        if cache.get(&current_pos).is_some() {
            return *cache.get(&current_pos).unwrap();
        }
        let mut possible_positions = vec![];
        // left
        if current_pos.0 != 0 && height_map[current_pos.1][current_pos.0 - 1] <= current_height + 1
        {
            possible_positions.push((current_pos.0 - 1, current_pos.1))
        }
        // right
        if current_pos.0 + 1 < height_map[0].len()
            && height_map[current_pos.1][current_pos.0 + 1] <= current_height + 1
        {
            possible_positions.push((current_pos.0 + 1, current_pos.1))
        }

        // up
        if current_pos.1 != 0 && height_map[current_pos.1 - 1][current_pos.0] <= current_height + 1
        {
            possible_positions.push((current_pos.0, current_pos.1 - 1))
        }
        // down
        if current_pos.1 + 1 < height_map.len()
            && height_map[current_pos.1 + 1][current_pos.0] <= current_height + 1
        {
            possible_positions.push((current_pos.0, current_pos.1 + 1))
        }
        possible_positions = possible_positions
            .iter()
            .filter(|pos| !path.contains(*pos))
            .map(|v| *v)
            .collect();
        possible_positions = possible_positions.iter().map(|v| *v).collect();
        if possible_positions.len() == 0 {
            cache.insert(current_pos, 9999);
            return 9999;
        }

        let result = possible_positions
            .iter()
            .map(|pos| {
                let mut new_path = path.to_owned();
                new_path.push(current_pos);
                traverse(height_map.to_owned(), end_pos, *pos, new_path, cache)
            })
            .min()
            .unwrap();
        cache.insert(current_pos, result);

        return result;
    }

    println!("S: {start_pos:?} E: {end_pos:?}");
    // let path = Vec::new();
    // let mut h = height_map.to_owned();
    // h[end_pos.unwrap().1][end_pos.unwrap().0] = 'z' as i16;
    // let mut cache = HashMap::new();
    // let result = traverse(
    //     height_map,
    //     end_pos.unwrap(),
    //     start_pos.unwrap(),
    //     path,
    //     &mut cache,
    // );
    // println!("result: {}", result);
    //
    fn bfs(
        start_pos: Option<(usize, usize)>,
        end_pos: Option<(usize, usize)>,
        height_map: Vec<Vec<i16>>,
    ) -> u64 {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((0, start_pos.unwrap()));
        while let Some((distance, (x, y))) = queue.pop_front() {
            for (nx, ny) in [
                (x as i16 + 1, y as i16),
                (x as i16 - 1, y as i16),
                (x as i16, y as i16 + 1),
                (x as i16, y as i16 - 1),
            ]
            .iter()
            {
                if *nx >= 0
                    && *ny >= 0
                    && *nx < height_map[0].len() as i16
                    && *ny < height_map.len() as i16
                    && height_map[*ny as usize][*nx as usize] <= height_map[y][x] + 1
                    && !visited.contains(&(*nx, *ny))
                {
                    visited.insert((*nx, *ny));
                    if (*nx as usize, *ny as usize) == end_pos.unwrap() {
                        println!("found solution {}", distance + 1);
                        return distance + 1;
                    }
                    queue.push_back((distance + 1, (*nx as usize, *ny as usize)))
                }
            }
        }
        return 100000;
    }
    height_map[start_pos.unwrap().1][start_pos.unwrap().0] = 0;
    height_map[end_pos.unwrap().1][end_pos.unwrap().0] = 25;

    println!("{}", bfs(start_pos, end_pos, height_map.to_owned()));

    let mut min = 9999;
    for y in 0..height_map.to_owned().len() {
        for x in 0..height_map.to_owned().len() {
            if height_map[y][x] == 0 {
                let r = bfs(Some((x, y)), end_pos, height_map.to_owned());
                if (r < min) {
                    min = r;
                }
            }
        }
    }

    println!("part 2 min: {}", min);

    Ok(())
}
