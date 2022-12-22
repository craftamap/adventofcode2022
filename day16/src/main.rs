use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    rc::{Rc, Weak},
    sync::Mutex,
};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    _ = main1();
    // _ = main2();
}

struct ParsedValve {
    id: String,
    flow_rate: usize,
    tunnels_to: Vec<String>,
}

#[derive(Debug)]
struct Valve {
    id: String,
    flow_rate: usize,
    open: bool,
    tunnels_to: Vec<Weak<RefCell<Valve>>>,
}

fn main1() -> Result<(), Box<dyn Error>> {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut pv = vec![];
    for l in reader.lines() {
        let line = l.unwrap();
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"Valve (..) has flow rate=(\d*); .* valves? (.*)").unwrap();
        }
        let captures = RE.captures(&line).unwrap();
        let id = captures.get(1).unwrap().as_str().to_string();
        let rate = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let tunnels_to = captures
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|m| m.to_string())
            .collect::<Vec<String>>();
        pv.push(ParsedValve {
            id,
            flow_rate: rate,
            tunnels_to,
        })
    }

    let mut graph: HashMap<String, Rc<RefCell<Valve>>> = HashMap::new();
    for item in pv.iter() {
        graph.insert(
            item.id.to_owned(),
            Rc::new(RefCell::new(Valve {
                id: item.id.to_owned(),
                flow_rate: item.flow_rate,
                open: false,
                tunnels_to: vec![],
            })),
        );
    }
    for item in pv.iter() {
        let i = graph.get(&item.id).unwrap();
        for tunnel_target in item.tunnels_to.iter() {
            let dg = Rc::downgrade(&graph.get(tunnel_target).unwrap().to_owned());
            let mut j = i.borrow_mut();
            j.tunnels_to.push(dg);
        }
    }

    let mut stack = vec![];
    stack.push((0 as isize, 30 as isize, vec!["AA".to_owned()]));
    let mut max = vec![];
    while stack.len() > 0 {
        let (pressure, remaining_steps, visited) = stack.pop().unwrap();
        let current = visited.iter().last().unwrap().to_owned();
        if remaining_steps < 0 {
            max.push(pressure);
            continue;
        }
        for node in graph.values() {
            if visited.contains(&node.to_owned().borrow().id.to_owned()) {
                continue;
            }

            if node.to_owned().borrow().flow_rate == 0 {
                max.push(pressure);
                continue;
            }

            let l = shortest_path_between(
                &graph,
                current.to_owned(),
                node.to_owned().borrow().id.to_owned(),
            ) as isize;

            let steps = l;
            let pressure_released =
                (remaining_steps - steps) as isize * node.to_owned().borrow().flow_rate as isize;

            let mut v = visited.to_owned();
            v.push(node.to_owned().borrow().id.to_owned());
            stack.push((pressure + pressure_released, remaining_steps - steps, v));

            // println!("{:?} score: {}", node.to_owned().borrow().id, pressure_released,)
        }
    }

    println!("{}", max.iter().max().unwrap());

    Ok(())
}

lazy_static! {
    static ref CACHE: Mutex<HashMap<String, usize>> = Mutex::new(HashMap::new());
}

fn shortest_path_between(
    graph: &HashMap<String, Rc<RefCell<Valve>>>,
    a: String,
    b: String,
) -> usize {
    let map = CACHE.lock().unwrap();
    let r = map.get(&(a.to_owned() + &b.to_owned()));
    if r.is_some() {
        return *r.unwrap();
    }
    drop(map);

    let mut queue = VecDeque::new();
    queue.push_front((a.to_owned(), vec![a.to_owned()]));
    let mut visited = HashSet::new();
    while queue.len() > 0 {
        let (node, path) = queue.pop_front().unwrap();
        visited.insert(node.to_owned());
        for child_node in graph.get(&node).unwrap().borrow().tunnels_to.iter() {
            let child_id = child_node
                .to_owned()
                .upgrade()
                .unwrap()
                .borrow()
                .id
                .to_owned();
            if child_id == b {
                let mut p = path.to_owned();
                p.push(child_id);
                let mut map = CACHE.lock().unwrap();
                map.insert(a.to_owned() + &b.to_owned(), p.len());
                // println!("{map:?}");
                return p.len();
            }

            if !visited.contains(&child_id) {
                visited.insert(child_id.to_owned());
                let mut p = path.to_owned();
                p.push(child_id.to_owned());
                queue.push_back((child_id, p));
            }
        }
    }
    return 999;
}
