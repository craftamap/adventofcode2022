use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

use std::hash::{Hash, Hasher};

fn main() {
    _ = main1();
    // _ = main2();
}

#[derive(Debug, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x, self.y).hash(state)
    }
}

#[derive(Debug, Clone)]
struct ParseResult {
    sensor: Point,
    beacon: Point,
}

fn parse_line(line: String) -> ParseResult {
    let re =
        Regex::new(r"Sensor at x=(-?\d*), y=(-?\d*): closest beacon is at x=(-?\d*), y=(-?\d*)")
            .unwrap();
    let captur = re.captures(&line).unwrap();

    ParseResult {
        sensor: Point {
            x: captur[1].parse().unwrap(),
            y: captur[2].parse().unwrap(),
        },
        beacon: Point {
            x: captur[3].parse().unwrap(),
            y: captur[4].parse().unwrap(),
        },
    }
}

fn manhattan(p1: &Point, p2: &Point) -> isize {
    return (p1.x - p2.x).abs() + (p1.y - p2.y).abs();
}

#[derive(Debug)]
enum Field {
    Sensor,
    Beacon,
    Blocked,
}

fn blocked_points(result: &ParseResult) -> HashSet<Point> {
    let mut points = HashSet::new();
    let distance = manhattan(&result.sensor, &result.beacon);
    println!("{distance}");
    // println!("sensor: {:?}, distance: {distance}", &result.sensor);
    let first_y = result.sensor.y - distance;
    for i in 0..=distance * 2 {
        if i <= distance {
            for j in 0..=i {
                points.insert(Point {
                    y: first_y + i,
                    x: result.sensor.x + j,
                });
                points.insert(Point {
                    y: first_y + i,
                    x: result.sensor.x - j,
                });
            }
        } else {
            for j in 0..=distance * 2 - i {
                points.insert(Point {
                    y: first_y + i,
                    x: result.sensor.x + j,
                });
                points.insert(Point {
                    y: first_y + i,
                    x: result.sensor.x - j,
                });
            }
        }
    }
    // println!("blocked: {points:?}")
    points
}

fn main1() -> Result<(), Box<dyn Error>> {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut map = HashMap::new();

    for l in reader.lines() {
        let line = l.unwrap();
        let line_points = parse_line(line);
        map.insert(line_points.to_owned().sensor, Field::Sensor);
        map.insert(line_points.to_owned().beacon, Field::Beacon);
        let points = blocked_points(&line_points);
        println!("{} blocked_points", points.len());
        for point in points.iter() {
            map.entry(point.to_owned()).or_insert(Field::Blocked);
        }
    }

    println!("{map:?}");


    // Idea: check the distance to each sensor/beacon pair - if smaller
    let cnt = map
        .into_iter()
        .filter(|k| k.0.y == 10 && matches!(k.1, Field::Blocked))
        .count();
    println!("{cnt}");

    Ok(())
}
