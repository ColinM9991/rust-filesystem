use crate::filesystem::FileSystem;

pub fn parse(input: &str) -> Result<FileSystem, String> {
    let mut file_system = FileSystem::new();

    let lines = input
        .trim()
        .lines()
        .map(|line| line.trim().split_whitespace().collect::<Vec<_>>());

    for line in lines {
        match line[..] {
            ["$", "cd", arg] => file_system.change_dir(arg)?,
            ["$", "ls"] => continue,
            ["dir", name] => {
                file_system.create_directory(name);
            }
            [size, name] => {
                let size = size.parse::<u64>().map_err(|e| "Invalid size specified")?;
                file_system.create_file(name, size);
            }
            _ => continue,
        }
    }

    Ok(file_system)
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

        let size = file_system.get_size();

        assert_eq!(size, 48381165);
    }
}
