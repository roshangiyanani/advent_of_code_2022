use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Node {
    File(File),
    Dir(Dir),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct File {
    size: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Dir {
    children: HashMap<String, Node>,
}

impl Dir {
    fn root() -> Dir {
        Dir {
            children: HashMap::new(),
        }
    }

    fn insert_file(&mut self, name: &str, size: usize) -> Result<&mut File, String> {
        let entry = self.children.entry(name.to_owned());
        match entry {
            Entry::Occupied(_) => Err(format!("dir already contains entry for {}", name)),
            Entry::Vacant(v) => match v.insert(Node::File(File { size })) {
                Node::File(file) => Ok(file),
                Node::Dir(_) => {
                    unreachable!("we created it as a file -- how are we getting a dir?")
                }
            },
        }
    }

    fn insert_dir(&mut self, name: &str) -> Result<&mut Dir, String> {
        let entry = self.children.entry(name.to_owned());
        match entry {
            Entry::Occupied(_) => Err(format!("dir already contains entry for {}", name)),
            Entry::Vacant(v) => {
                let children = HashMap::new();
                match v.insert(Node::Dir(Dir { children })) {
                    Node::File(_) => {
                        unreachable!("we created it as a dir -- how are we getting a file?")
                    }
                    Node::Dir(dir) => Ok(dir),
                }
            }
        }
    }

    fn get_child(&self, name: &str) -> Option<&Node> {
        self.children.get(name)
    }

    fn get_child_mut(&mut self, name: &str) -> Option<&mut Node> {
        self.children.get_mut(name)
    }

    fn get_recursive_dir<S: AsRef<str>>(&self, path: &[S]) -> &Dir {
        let mut current = self;
        for dir in path.iter() {
            match current.get_child(dir.as_ref()) {
                Some(Node::Dir(dir)) => current = dir,
                Some(Node::File(_)) => panic!("expected dir, not file at '{}'", dir.as_ref()),
                None => panic!("dir '{}' must exist", dir.as_ref()),
            };
        }

        current
    }

    fn get_recursive_dir_mut<'a, S: AsRef<str>>(&'a mut self, path: &[S]) -> &'a mut Dir {
        let mut current = self;
        for dir in path.iter() {
            match current.get_child_mut(dir.as_ref()) {
                Some(Node::Dir(dir)) => current = dir,
                Some(Node::File(_)) => panic!("expected dir, not file at '{}'", dir.as_ref()),
                None => panic!("dir '{}' must exist", dir.as_ref()),
            };
        }

        current
    }

    fn pretty_print(&self, f: &mut Formatter<'_>, indent: u8) -> std::fmt::Result {
        for (name, node) in self
            .children
            .iter()
            .sorted_by_key(|(name, _)| name.as_str())
        {
            for _ in 0..indent {
                write!(f, "  ")?;
            }
            write!(f, "- {} ", name)?;

            match node {
                Node::File(File { size }) => writeln!(f, "(file, size={})", size)?,
                Node::Dir(dir) => {
                    writeln!(f, "(dir)")?;
                    dir.pretty_print(f, indent + 1)?;
                }
            }
        }
        Ok(())
    }

    fn folder_size(&self) -> usize {
        self.children
            .iter()
            .map(|(_, node)| match node {
                Node::File(File { size }) => *size,
                Node::Dir(dir) => dir.folder_size(),
            })
            .sum()
    }

    fn smallest_folder_at_least(&self, n: usize) -> Result<(Option<&str>, usize), usize> {
        let mut total = 0;
        let mut best: Option<(&str, usize)> = None;

        for (name, child) in self.children.iter() {
            match child {
                Node::File(File { size }) => total += *size,
                Node::Dir(dir) => {
                    match (best, dir.smallest_folder_at_least(n)) {
                        (None, Err(size)) => total += size,
                        (None, Ok((None, size))) => best = Some((name, size)),
                        (None, Ok((Some(name), size))) => best = Some((name, size)),
                        (Some(_), Err(_)) => (), // don't need to total it, since we're not going to use total
                        (Some((_, old)), Ok((_, new))) if old <= new => (),
                        (Some(_), Ok((None, size))) => best = Some((name, size)),
                        (Some(_), Ok((Some(name), size))) => best = Some((name, size)),
                    }
                }
            }
        }

        match best {
            Some((name, size)) => Ok((Some(name), size)),
            None if total >= n => Ok((None, total)),
            None => Err(total),
        }
    }

    fn sum_folder_sizes_below<const N: usize>(&self) -> (usize, usize) {
        let mut folder_size = 0;
        let mut sum_folder_size_below_n = 0;
        for (_, child) in self.children.iter() {
            match child {
                Node::File(File { size }) => folder_size += size,
                Node::Dir(dir) => {
                    let (size, sum) = dir.sum_folder_sizes_below::<N>();
                    folder_size += size;
                    sum_folder_size_below_n += sum;
                }
            }
        }

        if folder_size < N {
            sum_folder_size_below_n += folder_size;
        }

        (folder_size, sum_folder_size_below_n)
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "- / (dir)")?;
        self.pretty_print(f, 1)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum CdTarget<'a> {
    Root,
    Parent,
    Child(&'a str),
}

impl<'a> From<&'a str> for CdTarget<'a> {
    fn from(target: &'a str) -> Self {
        match target {
            "/" => CdTarget::Root,
            ".." => CdTarget::Parent,
            _ => CdTarget::Child(target),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Command<'a> {
    Cd(CdTarget<'a>),
    List,
}

impl<'a> TryFrom<&'a str> for Command<'a> {
    type Error = String;

    fn try_from(command: &'a str) -> Result<Self, Self::Error> {
        if command == "$ ls" {
            Ok(Command::List)
        } else if let Some(target) = command.strip_prefix("$ cd ") {
            Ok(Command::Cd(CdTarget::from(target)))
        } else {
            Err(format!("unrecognized command: '{}'", command))
        }
    }
}

#[aoc_generator(day7)]
pub fn parse_root_from_commands(input: &str) -> Dir {
    let mut lines = input.lines().peekable();

    assert_eq!(lines.next(), Some("$ cd /"));
    let mut root = Dir::root();
    let mut path: Vec<String> = Vec::new();
    let mut current: &mut Dir = &mut root;

    while let Some(command) = lines.next() {
        let command = Command::try_from(command).expect("could not parse input");
        match command {
            Command::Cd(target) => {
                match target {
                    CdTarget::Root => {
                        path.clear();
                        current = &mut root;
                    }
                    CdTarget::Parent => {
                        path.pop();

                        current = root.get_recursive_dir_mut(&path);
                    }
                    CdTarget::Child(child) => match current.get_child_mut(child) {
                        None => panic!("no child for {}", child),
                        Some(Node::File(_)) => panic!("expected dir, found file '{}'", child),
                        Some(Node::Dir(dir)) => {
                            path.push(child.to_owned());
                            current = dir;
                        }
                    },
                };
            }
            Command::List => {
                while let Some(line) = lines.peek() {
                    if line.starts_with('$') {
                        break;
                    }

                    let line = lines.next().unwrap();
                    if let Some(dir) = line.strip_prefix("dir ") {
                        current.insert_dir(dir).unwrap();
                    } else if let Some((size, name)) = line.split_once(' ') {
                        let size = size.parse::<usize>().unwrap();
                        current.insert_file(name, size).unwrap();
                    } else {
                        panic!("unexpected ls line: '{}'", line);
                    }
                }
            }
        }
    }

    root
}

#[aoc(day7, part1)]
pub fn sum_folder_sizes_below_100000(root: &Dir) -> usize {
    let (_, sum) = root.sum_folder_sizes_below::<100000>();
    sum
}

#[aoc(day7, part2)]
pub fn smallest_folder_to_delete_for_update(root: &Dir) -> String {
    const TOTAL_DISK_AVAILABLE: usize = 70_000_000;
    const FREE_SPACE_REQUIRED: usize = 30_000_000;

    let total_size = root.folder_size();
    let free_space = TOTAL_DISK_AVAILABLE - total_size;
    let space_to_clear = FREE_SPACE_REQUIRED - free_space;

    let (name, size) = root
        .smallest_folder_at_least(space_to_clear)
        .expect("could not find folder above size 30_000_000");
    format!("folder '{}' of size {}", name.unwrap_or("/"), size)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_parse_dir_from_commands() {
        const EXPECTED: &str = "\
- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - d.ext (file, size=5626152)
    - d.log (file, size=8033020)
    - j (file, size=4060174)
    - k (file, size=7214296)
";

        let root = parse_root_from_commands(INPUT);
        assert_eq!(format!("{}", root), EXPECTED);
    }

    #[test]
    fn test_folder_size() {
        let root = parse_root_from_commands(INPUT);
        assert_eq!(root.folder_size(), 48381165);
        assert_eq!(root.get_recursive_dir(&["d",]).folder_size(), 24933642);
        assert_eq!(root.get_recursive_dir(&["a",]).folder_size(), 94853);
        assert_eq!(root.get_recursive_dir(&["a", "e"]).folder_size(), 584)
    }

    #[test]
    fn test_part_one() {
        let root = parse_root_from_commands(INPUT);
        let sum = sum_folder_sizes_below_100000(&root);
        assert_eq!(sum, 95437);
    }

    #[test]
    fn test_part_two() {
        let root = parse_root_from_commands(INPUT);
        let result = smallest_folder_to_delete_for_update(&root);
        assert_eq!(result, "folder 'd' of size 24933642");
    }
}
