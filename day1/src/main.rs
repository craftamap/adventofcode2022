use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut vec = Vec::<u64>::new();

    let mut index = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        if l.len() == 0 {
            println!("empty line");
            index += 1;
            continue
        }
        if vec.len() <= index {
            vec.push(0)
        }
        let new_number = vec.get(index).or(Some(&0u64)).unwrap() + l.parse::<u64>()?;
        println!("new_number {}", new_number);
        vec[index] = new_number;
    }

    vec.sort_unstable();
    vec.reverse();

    println!("sorted reversed {:?}", vec);
    println!("sorted reversed biggest {}", vec[0]);
    // part 2
    println!("sorted reversed biggest three {}", vec[0] + vec[1] + vec[2]);

    Ok(())
}
