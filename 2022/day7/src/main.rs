use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

use std::io::{self, BufRead};

#[derive(Clone, Debug)]
enum FileType {
    Dir,
    File(usize),
}

#[derive(Clone, Debug)]
struct Tree {
    name: String,
    children: HashMap<String, Rc<RefCell<Tree>>>,
    parent: Option<Rc<RefCell<Tree>>>,
    filetype: FileType,
}

impl Tree {
    pub fn size(&self) -> usize {
        //let s = self.children.iter().filter_map(|f| if let FileType::File(size) = f.filetype { Some(size) } else { None}).sum();
        let mut total = 0;

        for node in self.children.values() {
            let node = node.borrow();
            match node.filetype {
                FileType::File(s) => total += s,
                FileType::Dir => total += node.size(),
            }
        }

        total
    }
}

fn main() {
    let root = Tree {
        name: String::from("/"),
        children: HashMap::new(),
        parent: None,
        filetype: FileType::Dir,
    };

    let root: Rc<RefCell<Tree>> = Rc::new(RefCell::new(root));

    let mut pwd = &root.clone(); 


    for line in io::stdin().lock().lines().flatten() {
        println!("command: {line}");
        dbg!(&pwd);
        let tokens: Vec<_> = line.split_whitespace().collect();

        if tokens[0] == "$" && tokens[1] == "cd" {
            if tokens[2] == "/" {
                pwd = &root.clone();
            } else if tokens[2] == ".." {
                // could panic if parent is null
                let new_pwd = pwd.borrow();
                let parent = new_pwd.parent.as_ref().unwrap();
                pwd = &parent.clone();
            } else {
                let p = pwd;
                let new_pwd = Rc::try_unwrap(*p).unwrap().into_inner().parent.unwrap();
                pwd = &new_pwd;
            }
        } else if tokens[0] == "dir" {
            let new_dir = Tree {
                name: tokens[1].into(),
                children: HashMap::new(),
                parent: Some(pwd.clone()),
                filetype: FileType::Dir,
            };

            let new_dir = Rc::new(RefCell::new(new_dir));

            pwd.borrow_mut().children.insert(tokens[1].into(), new_dir);
            println!("  created dir '{}'", tokens[1]);
        } else {
            println!("  todo add file to dir");
        }
    }
}
