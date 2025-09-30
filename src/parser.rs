use crate::filesystem::FileSystem;
use crate::node::{NodeRef, NodeType};

pub fn parse(input: &str) -> Result<FileSystem, String> {
    let mut file_system = FileSystem::new();

    let lines: Vec<Vec<&str>> = input
        .trim()
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    for line in lines {
        match line[..] {
            ["$", "cd", arg] => file_system.change_dir(arg)?,
            ["$", "ls"] => continue,
            ["dir", name] => {
                file_system.create_directory(name);
            }
            [size, name] => {
                let size = size.parse::<u64>().map_err(|_| "Invalid size specified")?;
                file_system.create_file(name, size);
            }
            _ => continue,
        }
    }

    Ok(file_system)
}

pub fn get_sizes(node: &NodeRef) -> Vec<u64> {
    let node = node.borrow();
    let mut size: Vec<u64> = vec![node.get_size()];

    if let NodeType::Directory { children } = &node.node_type {
        let mut child_sizes = children
            .iter()
            .filter(|e| e.borrow().is_directory())
            .flat_map(|e| get_sizes(e))
            .collect::<Vec<_>>();
        size.append(&mut child_sizes);
    }

    size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
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

        let file_system = parse(INPUT);

        assert!(file_system.is_ok());

        let file_system = file_system.unwrap();

        let size: u64 = file_system.get_size();

        assert_eq!(size, 48381165);
    }

    #[test]
    fn test_parse_solution_1() {
        let input = include_str!("../files/day7.txt");

        let file_system = parse(input);

        assert!(file_system.is_ok());

        let file_system = file_system.unwrap();

        let size: u64 = get_sizes(&file_system.get_root())
            .into_iter()
            .filter(|&e| e <= 100000)
            .sum();

        assert_eq!(size, 1513699);
    }

    #[test]
    fn test_parse_solution_2() {
        let input = include_str!("../files/day7.txt");

        let file_system = parse(input);

        assert!(file_system.is_ok());

        let file_system = file_system.unwrap();

        const MAX_SPACE: u64 = 70_000_000;
        const REQUIRED_SPACE: u64 = 30_000_000;

        let size: u64 = file_system.get_size();

        let remaining_space = MAX_SPACE - size;
        let space_to_clear = REQUIRED_SPACE - remaining_space;

        let size = get_sizes(&file_system.get_root())
            .into_iter()
            .filter(|&e| e >= space_to_clear)
            .min()
            .unwrap();

        assert_eq!(size, 7991939);
    }
}
