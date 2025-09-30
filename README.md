# Description

Being relatively new to Advent of Code I thought I'd use it to learn Rust - which was a big mistake.

Advent of Code 2022 - Day 7 introduced a problem that piqued my interest as it seemed like a perfect problem for the
very powerful enums in Rust.

When I tackled this first back in 2022, I was (and still am) new to Rust and so I struggled with the filesystem concept
due to Rusts borrow-checker. Eventually I did complete the challenge with some messy code and
a [glaringly obvious memory leak](https://github.com/ColinM9991/AdventOfCode/blob/8515def2b4af8bab2e2e254b387a23b71eb237b6/aoc_2022/src/day7.rs#L15).

Having lost interest in Rust for a period of time (lack of projects to work on), I decided to tackle this problem the
right way by using `Weak<T>` and having better separation of concerns while also working with the borrow-checker a bit
better.

- `NodeType` is an enum that separates file sizes and directory items
- `Node` represents an object within a filesystem. A file or a directory. It exposes some helper methods for recursively
  fetching the size of a directory tree or a file.
- `FileSystem` is an abstraction around a tree structure. It contains a root and tracks the current working directory
    - A file or directory can be created with `FileSystem::create_file(fs, name, size)` or
      `FileSystem::create_directory(fs, name)` respectively
    - the current working directory can be changed via `FileSystem::change_dir(fs, name)`
    - a file or directory can be added to a subdirectory once the file system has changed the working directory into the
      subdirectory
- `parser` contains the advent of code specific logic
    - `parse` translates the advent of code input to a series of invocations on a `FileSystem` as per the rules of the
      input
        - `dir x` creates a directory named x
        - `$ cd y` invokes `change_dir` to set the current working directory to `y` (as long as it exists in the tree)
        - `10582 file.txt` creates a file within the current working directory. 10582 is the file size and file.txt is
          the name.