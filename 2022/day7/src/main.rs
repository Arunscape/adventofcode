use std::collections::HashMap;
use std::path::PathBuf;

use std::io::{self, BufRead};

#[derive(Clone, Debug)]
enum FileType {
    Dir,
    File(usize),
}

#[derive(Clone, Debug)]
struct Tree {
    name: String,
    children: HashMap<String, *mut Tree>,
    parent: Option<*mut Tree>,
    filetype: FileType,
}

impl Tree {
    pub fn size(&self) -> usize {
        //let s = self.children.iter().filter_map(|f| if let FileType::File(size) = f.filetype { Some(size) } else { None}).sum();
        let mut total = 0;

        for &node in self.children.values() {
            unsafe {
                match (*node).filetype {
                    FileType::File(s) => total += s,
                    FileType::Dir => total += (*node).size(),
                }
            }
        }

        total
    }

    pub fn get_all_dirs(&mut self) -> Vec<usize> {
        match self.filetype {
            FileType::File(_) => Vec::new(),
            FileType::Dir => {
                let mut ret = vec![self.size()];

                for &node in self.children.values() {
                    unsafe {
                        match (*node).filetype {
                            FileType::File(_) => {}
                            FileType::Dir => {
                                ret.append(&mut (*node).get_all_dirs());
                            }
                        }
                    }
                }
                ret
            }
        }
    }
}

fn main() {
    let mut root = Tree {
        name: String::from("/"),
        children: HashMap::new(),
        parent: None,
        filetype: FileType::Dir,
    };

    let mut pwd = &mut root as *mut Tree;
    let root = &mut root as *mut Tree;

    for line in io::stdin().lock().lines().flatten() {
        println!("command: {line}");
        unsafe {
            println!("  pwd: {}", (*pwd).name);
        }
        let tokens: Vec<_> = line.split_whitespace().collect();
        if tokens.len() < 2 {
            continue;
        }

        if tokens[0] == "$" && tokens[1] == "cd" {
            if tokens[2] == "/" {
                pwd = root as *mut Tree;
            } else if tokens[2] == ".." {
                // could panic if parent is null
                pwd = unsafe { (*pwd).parent.unwrap() };
            } else {
                pwd = unsafe { (*pwd).children[tokens[2]] }
            }
        } else if tokens[0] == "dir" {
            let new_dir = Tree {
                name: tokens[1].into(),
                children: HashMap::new(),
                parent: Some(pwd),
                filetype: FileType::Dir,
            };

            let new_dir = Box::into_raw(Box::new(new_dir));

            unsafe { (*pwd).children.insert(tokens[1].into(), new_dir) };
            println!("  created dir '{}'", tokens[1]);
        } else if tokens[0] == "$" && tokens[1] == "ls" {
        } else {
            let size: usize = tokens[0].parse().unwrap();
            let new_file = Tree {
                name: tokens[1].into(),
                children: HashMap::new(),
                parent: Some(pwd),
                filetype: FileType::File(size),
            };

            let new_file = Box::into_raw(Box::new(new_file));
            unsafe { (*pwd).children.insert(tokens[1].into(), new_file) };
            println!("  added file to pwd '{}'", tokens[1]);
        }
    }

    let rootsize = unsafe { (*root).size() };
    dbg!(&rootsize);

    let mut at_most = Vec::new();
    for &node in unsafe { (*root).children.values() } {
        unsafe {
            match (*node).filetype {
                FileType::File(_) => {}
                FileType::Dir => {
                    let s = (*node).size();
                    if s <= 100000 {
                        at_most.push(s);
                    }
                }
            }
        }
    }

    let dirs = unsafe {(*root).get_all_dirs()};

    let sum: usize = dirs.iter().filter(|&s| s < &100000).sum();
    println!("part 1: {}", sum);

    let total = 70000000;
    let unused = 30000000;

    let x: usize = *dirs.iter().filter(|&s| (total - rootsize) + s >= unused).min().unwrap();
    println!("part 2: {}", x);

}
