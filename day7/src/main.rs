use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

use std::ops::Deref;

fn main() {
    _ = main1();
    // _ = main2();
}

#[derive(Debug)]
enum Line {
    CD { directory: String },
    LS,
    DIR { name: String },
    FILE { name: String, size: u64 },
}

fn parse_line(line: String) -> Line {
    if line.starts_with("$") {
        if line.starts_with("$ ls") {
            Line::LS
        } else {
            Line::CD {
                directory: line.strip_prefix("$ cd ").unwrap().to_string(),
            }
        }
    } else {
        if line.starts_with("dir") {
            Line::DIR {
                name: line.strip_prefix("dir ").unwrap().to_string(),
            }
        } else {
            let mut splitted = line.splitn(2, " ");
            let size: u64 = splitted.next().unwrap().parse().unwrap();
            let name = splitted.next().unwrap();

            Line::FILE {
                name: name.to_string(),
                size,
            }
        }
    }
}

#[derive(Debug)]
enum FSNode {
    File {
        parent: Rc<RefCell<FSNode>>,
        name: String,
        size: u64,
    },
    Directory {
        parent: Option<Rc<RefCell<FSNode>>>,
        name: String,
        children: RefCell<HashMap<String, Rc<RefCell<FSNode>>>>,
    },
}

impl FSNode {
    fn print(&self, d: u64) {
        match self {
            FSNode::File { parent, name, size } => {
                println!("{} name: {}, size: {}", "  ".repeat(d as usize), name, size)
            }
            FSNode::Directory {
                parent,
                name,
                children,
            } => {
                println!("{} name: {}", "  ".repeat(d as usize), name);
                for c in children.borrow().values() {
                    let a = Rc::clone(c);
                    let b = a.borrow();
                    let c = b.deref();
                    c.print(d + 1);
                }
            }
        }
    }
    fn size(&self) -> u64 {
        match self {
            FSNode::File { parent, name, size } => {
                println!("name: {}, size: {}", name, size);
                return size.clone();
            }
            FSNode::Directory {
                parent,
                name,
                children,
            } => {
                let mut sum = 0;
                for c in children.borrow().values() {
                    let a = Rc::clone(c);
                    let b = a.borrow();
                    let c = b.deref();
                    sum += c.size();
                }
                println!("dir name: {}, size: {}", name, sum);
                sum
            }
        }
    }
    fn size_array(&self) -> (u64, Option<Vec<u64>>) {
        match self {
            FSNode::File { parent, name, size } => {
                println!("name: {}, size: {}", name, size);
                return (size.clone(), None);
            }
            FSNode::Directory {
                parent,
                name,
                children,
            } => {
                let mut sum = 0;
                let mut targetVec = vec![];
                for c in children.borrow().values() {
                    let a = Rc::clone(c);
                    let b = a.borrow();
                    let c = b.deref();
                    let (size, arr) = c.size_array();
                    sum += size;
                    if (arr.is_some()) {
                        targetVec.append(&mut arr.unwrap());
                    }
                }
                targetVec.push(sum);
                println!(
                    "dir name: {}, size: {}, targetVec: {:?}",
                    name, sum, targetVec
                );
                return (sum.clone(), Some(targetVec));
            }
        }
    }
}

fn main1() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let root = Rc::new(RefCell::new(FSNode::Directory {
        parent: None,
        name: "root".to_string(),
        children: RefCell::new(HashMap::new()),
    }));
    let mut current_dir = Rc::clone(&root);
    for l in reader.lines() {
        let pl = parse_line(l.unwrap());
        println!("{:?}", pl);
        match pl {
            Line::CD { directory } => {
                if (directory == "/") {
                    current_dir = Rc::clone(&root);
                } else if (directory == "..") {
                    let new_dir = if let FSNode::Directory {
                        parent,
                        name,
                        children,
                    } = &*current_dir.borrow()
                    {
                        Rc::clone(&parent.as_ref().unwrap())
                    } else {
                        unreachable!()
                    };
                    current_dir = new_dir;
                } else {
                    let a = match &*current_dir.borrow() {
                        FSNode::Directory {
                            parent: _,
                            name: _,
                            children,
                        } => Rc::clone(children.borrow().get(&directory).unwrap()),
                        FSNode::File {
                            parent: _,
                            name: _,
                            size: _,
                        } => unreachable!(),
                    };
                    current_dir = a;
                }
            }
            Line::LS => {}
            Line::DIR { name } => {
                let parent = Some(current_dir.clone());
                let cd = current_dir.borrow();
                if let FSNode::Directory {
                    parent: _,
                    name: _,
                    children,
                } = &*cd
                {
                    let a = Rc::new(RefCell::new(FSNode::Directory {
                        parent,
                        name: name.to_owned(),
                        children: RefCell::new(HashMap::new()),
                    }));
                    children.borrow_mut().insert(name.to_owned(), a)
                } else {
                    unreachable!()
                };
            }
            Line::FILE { name, size } => {
                let parent = current_dir.clone();
                let cd = current_dir.borrow();
                if let FSNode::Directory {
                    parent: _,
                    name: _,
                    children,
                } = &*cd
                {
                    let a = Rc::new(RefCell::new(FSNode::File {
                        parent: parent,
                        name: name.to_owned(),
                        size: size,
                    }));
                    children.borrow_mut().insert(name.to_owned(), a)
                } else {
                    unreachable!()
                };
            }
        };
    }

    println!();
    let sum = root
        .borrow()
        .size_array()
        .1
        .unwrap()
        .iter()
        .filter(|x| **x < 100000)
        .sum::<u64>();
    println!("part 1:");
    println!("sum: {}", sum);
    let free_space = 70000000 - root.borrow().size();
    let space_that_needs_to_be_freed: i128 = (30000000 - free_space).into();
    println!(
        "space_that_needs_to_be_freed: {}",
        space_that_needs_to_be_freed
    );
    let diff = root
        .borrow()
        .size_array()
        .1
        .unwrap()
        .iter()
        .map(|v| (*v, space_that_needs_to_be_freed - *v as i128))
        .filter(|(_, d)| *d < 0)
        .max_by_key(|(_, d)| d.clone());
    println!("diff {:?}", diff.unwrap());
    Ok(())
}
