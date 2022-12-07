use std::{
    collections::HashMap,
    io::stdin,
    sync::{Arc, Mutex},
};

enum Fd {
    Directory(Arc<Mutex<Dir>>),
    File(File),
}

struct Dir {
    name: String,
    parent: Option<Arc<Mutex<Dir>>>,
    fds: HashMap<String, Fd>,
    total_size: usize,
}

struct File {
    name: String,
    size: usize,
}

impl Dir {
    fn enter_dir(&mut self, parent: Arc<Mutex<Dir>>, name: &str) -> Arc<Mutex<Dir>> {
        match self.fds.get(name) {
            Some(Fd::Directory(dir)) => return dir.clone(),
            Some(Fd::File(file)) => {
                panic!("tried to cd into file")
            }
            None => {
                // New directory
                let new = Arc::new(Mutex::new(Dir {
                    name: name.to_string(),
                    parent: Some(parent),
                    fds: HashMap::new(),
                    total_size: 0,
                }));
                self.fds
                    .insert(name.to_string(), Fd::Directory(new.clone()));
                return new;
            }
        }
    }

    fn calculate_sum(&mut self, filter: usize) -> usize {
        let mut sum = if self.total_size <= filter {
            self.total_size
        } else {
            0
        };

        for fd in self.fds.values() {
            if let Fd::Directory(d) = fd {
                sum += d.lock().unwrap().calculate_sum(filter);
            }
        }

        sum
    }

    fn size_of_smallest_dir_larger_than(&self, min: usize) -> usize {
        let mut res = if self.total_size >= min {
            self.total_size
        } else {
            usize::MAX
        };

        for fd in self.fds.values() {
            if let Fd::Directory(dx) = fd {
                let d = dx.lock().unwrap();
                if d.total_size >= min {
                    res = usize::min(res, d.size_of_smallest_dir_larger_than(min))
                }
            }
        }
        res
    }
}

fn parent(dir: Arc<Mutex<Dir>>) -> Option<Arc<Mutex<Dir>>> {
    dir.lock().unwrap().parent.clone()
}

fn debug(dir: Arc<Mutex<Dir>>, ident: usize) {
    let d = dir.lock().unwrap();
    println!("- {} (dir, size={})", d.name, d.total_size);

    let i = " ".repeat(ident);
    for fd in d.fds.values() {
        match fd {
            Fd::File(f) => println!("{}{} (file, size={})", i, f.name, f.size),
            Fd::Directory(child) => {
                print!("{}", i);
                debug(child.clone(), ident + 2);
            }
        }
    }
}

fn main() {
    let root = Arc::new(Mutex::new(Dir {
        name: "/".to_string(),
        parent: None,
        fds: HashMap::new(),
        total_size: 0,
    }));
    let mut cwd = root.clone();

    for line in stdin().lines().flatten() {
        if line == "$ cd /" {
            while let Some(parent) = parent(cwd.clone()) {
                cwd = parent;
            }
        } else if line == "$ cd .." {
            cwd = parent(cwd.clone()).expect("parent exists")
        } else if let Some(entered_dir) = line.strip_prefix("$ cd ") {
            let child = cwd.lock().unwrap().enter_dir(cwd.clone(), entered_dir);
            cwd = child
        } else if line == "$ ls" {
            continue;
        } else if line.starts_with("dir ") {
        } else if let Some((size_str, name)) = line.split_once(' ') {
            let size: usize = str::parse(size_str).unwrap();
            let mut c = cwd.lock().unwrap();
            c.fds.insert(
                name.to_string(),
                Fd::File(File {
                    name: name.to_string(),
                    size,
                }),
            );
            c.total_size += size;

            let mut parent = c.parent.clone();
            while let Some(px) = parent {
                let mut p = px.lock().unwrap();
                p.total_size += size;
                parent = p.parent.clone();
            }
        }
    }

    debug(root.clone(), 2);

    let r = root.lock().unwrap();
    let used = r.total_size;
    let free = 70000000 - used;
    let need = 30000000 - free;
    println!("{}", r.size_of_smallest_dir_larger_than(need))
}
