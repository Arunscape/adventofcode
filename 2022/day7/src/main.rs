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
}

fn main() {
    let mut root = Tree {
        name: String::from("/"),
        children: HashMap::new(),
        parent:  None,
        filetype: FileType::Dir,
    };

    let mut pwd = &mut root as *mut Tree;
    let root = &root as *const Tree;

    for line in io::stdin().lock().lines().flatten() {
        println!("command: {line}");
        unsafe {
            println!("  pwd: {}", (*pwd).name);
        }
        let tokens: Vec<_> = line.split_whitespace().collect();

        if tokens[0] == "$" && tokens[1] == "cd" {
            if tokens[2] == "/" {
                pwd = root as *mut Tree;
            } else if tokens[2] == ".." {
                // could panic if parent is null
                pwd = unsafe { (*pwd).parent.unwrap() };
            } else {

                unsafe {
                dbg!(&(*pwd).children);
                }
                pwd = unsafe {(*pwd).children[tokens[2]]}
            }
        } else if tokens[0] == "dir" {
                let mut new_dir = Tree {
                    name: tokens[1].into(),
                    children: HashMap::new(),
                    parent: Some(pwd),
                    filetype: FileType::Dir,
                };



                unsafe { (*pwd).children.insert(tokens[1].into(), &mut new_dir) };
                println!("  created dir '{}'", tokens[1]);
        } else {
            println!("  todo add file to dir");
        }
    }
}
