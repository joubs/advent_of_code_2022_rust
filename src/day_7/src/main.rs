
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, not_line_ending,u64},
    combinator::{map},
    multi::{separated_list0, separated_list1},
    sequence::{preceded, separated_pair},
    IResult,
};

// Domain

#[derive(Debug, PartialEq, Clone)]
pub enum Entry {
    Dir { name: String },
    File { name: String, size: u64 },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Cd(String),
    Ls(Vec<Entry>),
}

#[derive(Debug)]
pub struct FileSystemItem {
    name: String, // fine for short strings of the problem here.
    size: u64,
    subdirs: Vec<FileSystemItem>,
}

impl FileSystemItem {
    pub fn rsize(&self) -> u64 {
        return self.size + self.subdirs.iter().map(|s| s.rsize()).sum::<u64>();
    }

    pub fn recursive_traversal(&self) -> Box<dyn Iterator<Item = &FileSystemItem> + '_> {
        return Box::new(
            std::iter::once(self).chain(
                self.subdirs
                    .iter()
                    .filter(|subdir| subdir.size == 0)
                    .flat_map(|subdir| subdir.recursive_traversal()),
            ),
        );
    }
}

fn build_fs_item_stack(commands: Vec<Command>) -> Vec<FileSystemItem> {
    let mut stack = vec![FileSystemItem {
        name: "/".into(),
        size: 0,
        subdirs: vec![],
    }];

    for command in commands {
        match command {
            Command::Cd(name) => match name.as_str() {
                "/" => {},
                ".." => {
                    let previous_dir = stack.pop().unwrap();
                    stack.last_mut().unwrap().subdirs.push(previous_dir)
                }
                _ => {
                    let new_dir = FileSystemItem{name: name.clone(), size: 0_u64, subdirs: vec![]};
                    stack.push(new_dir);
                }
            }
            Command::Ls(entries) => {
                for entry in entries {
                    match entry {
                        Entry::File{name, size} => {
                            let new_item = FileSystemItem{name, size, subdirs: vec![]};
                            stack.last_mut().unwrap().subdirs.push(new_item);
                        },
                        Entry::Dir { name } => {}

                    }
                }
            }
        }
    }


    return stack;
}

// Parsers

pub fn parse_input(input: &str) -> IResult<&str, Vec<Command>> {
    return separated_list1(line_ending, parse_command)(input);
}
pub fn parse_command(input: &str) -> IResult<&str, Command> {
    return preceded(tag("$ "), alt((parse_cd_command, parse_ls_command)))(input);
}

pub fn parse_cd_command(input: &str) -> IResult<&str, Command> {
    return map(preceded(tag("cd "), not_line_ending), |name| {
        Command::Cd(String::from(name))
    })(input);
}

pub fn parse_ls_command(input: &str) -> IResult<&str, Command> {
    return map(preceded(tag("ls\r\n"), parse_items), |items| {
        Command::Ls(items)
    })(input);
}

pub fn parse_items(input: &str) -> IResult<&str, Vec<Entry>> {
    return separated_list0(line_ending, parse_item)(input);
}

pub fn parse_item(input: &str) -> IResult<&str, Entry> {
    return alt((parse_file, parse_dir))(input);
}

pub fn parse_file(input: &str) -> IResult<&str, Entry> {
    return map(
        separated_pair(u64, tag(" "), not_line_ending),
        |(size, name)| Entry::File { name: String::from(name), size: size },
    )(input);
}


pub fn parse_dir(input: &str) -> IResult<&str, Entry> {
    return map(preceded(tag("dir "), alpha1), |name| Entry::Dir {
        name: String::from(name),
    })(input);
}

#[cfg(test)]
mod tests {
    use crate::{parse_cd_command, parse_command, parse_item, parse_ls_command, Command, Entry};

    #[test]
    fn parse_cd_works() {
        let command_input = "cd /toto";
        assert_eq!(
            parse_cd_command(command_input).unwrap().1,
            Command::Cd(String::from("/toto"))
        );
    }

    #[test]
    fn parse_item_works() {
        assert_eq!(parse_item("dir ls").unwrap().1, Entry::Dir { name: String::from("ls") });
        assert_eq!(
            parse_item("97889 rqpw.tex").unwrap().1,
            Entry::File {
                name: String::from("rqpw.tex"),
                size: 97889
            }
        );
    }

    #[test]
    fn parse_ls_works() {
        let command_input = "ls\r\n156 toto\r\ndir coucou";
        assert_eq!(
            parse_ls_command(command_input).unwrap().1,
            Command::Ls(vec![
                Entry::File {
                    name: String::from("toto"),
                    size: 156
                },
                Entry::Dir { name: String::from("coucou") }
            ])
        );
    }

    #[test]
    fn parse_command_works() {
        assert_eq!(parse_command("$ cd ls").unwrap().1, Command::Cd(String::from("ls")));
        //assert_eq!(parse_command("$ ls").unwrap().1, Command::Ls);
    }
}

fn main() {
    let commands = parse_input(include_str!(".\\real_input.txt")).unwrap().1;
    let mut stack = build_fs_item_stack(commands);

    let mut root = stack.pop().unwrap();
    while let Some(mut next) = stack.pop() {
        next.subdirs.push(root);
        root = next;
    }
    //dbg!(&root);

    let result_part_one = root.recursive_traversal()
        .map(|item| item.rsize())
        .filter(|&size| size < 100_000)
        //.inspect(|&i| { dbg!(i);})
        .sum::<u64>();

    dbg!(result_part_one);

    let used_space = root.rsize(); 
    let free_space = 70_000_000u64 - used_space; 
    let min_space = 30_000_000u64 - free_space; 
    let result_part_two = root.recursive_traversal()
        .map(|item| item.rsize())
        .filter(|&size| size > min_space)
        .min()
        .unwrap();
    
    dbg!(result_part_two);
}