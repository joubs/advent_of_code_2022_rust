use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, newline, space1, multispace1},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult,
};

// Domain
#[derive(Debug, Clone, PartialEq)]
pub struct Crate<'a>(&'a str);

#[derive(Debug, PartialEq)]
pub struct Instruction {
    move_: usize,
    from: usize,
    to: usize,
}

type Grid<'a> = Vec<Vec<Option<Crate<'a>>>>;

pub fn build_crate_stacks(grid_lines: Grid) -> Vec<Vec<Crate>> {
    let num_stacks = grid_lines.get(0).unwrap().len();
    let mut crate_stacks: Vec<Vec<Crate>> = vec![vec![]; num_stacks];

    for row in grid_lines.iter().rev() {
        for (index, value) in row.iter().enumerate() {
            if let Some(v) = value {
                crate_stacks[index].push(v.clone())
            }
        }
    }

    crate_stacks
}

pub fn part_one_rolling<'a>(crate_stacks: &Vec<Vec<Crate<'a>>>, instructions: &Vec<Instruction>) -> Vec<Vec<Crate<'a>>> 
{
    let mut res_crate_stacks = crate_stacks.clone();
    for inst in instructions{
        for _ in 0..inst.move_{
            let moving_crate = res_crate_stacks[inst.from - 1].pop().unwrap(); // we suppose the input is valid here
            res_crate_stacks[inst.to - 1].push(moving_crate);
        }
    }
    return res_crate_stacks
}

pub fn part_two_rolling<'a>(crate_stacks: &Vec<Vec<Crate<'a>>>, instructions: &Vec<Instruction>) -> Vec<Vec<Crate<'a>>> 
{
    let mut res_crate_stacks = crate_stacks.clone();
    for inst in instructions{
        let mut temp_vec: Vec<Crate> = vec![]; 
        
        for _ in 0..inst.move_{
            let moving_crate = res_crate_stacks[inst.from - 1].pop().unwrap(); // we suppose the input is valid here
            temp_vec.push(moving_crate);
        }
        for _ in 0..inst.move_ {
            res_crate_stacks[inst.to - 1].push(temp_vec.pop().unwrap());
        }
    }
    return res_crate_stacks
}

// Parsers

pub fn parse_input(input: &str) -> IResult<&str, (Grid, Vec<Instruction>)> {
    return separated_pair(
        parse_slot_grid,
        pair(parse_indices, multispace1),
        parse_instructions,
    )(input);
}

pub fn parse_crate(input: &str) -> IResult<&str, &str> {
    return delimited(char('['), alpha1, char(']'))(input);
}

pub fn parse_slot(input: &str) -> IResult<&str, Option<Crate>> {
    return map(alt((tag("   "), parse_crate)), |slot| match slot {
        "   " => None,
        new_crate => Some(Crate(new_crate)),
    })(input);
}

pub fn parse_slot_line(input: &str) -> IResult<&str, Vec<Option<Crate>>> {
    return separated_list0(tag(" "), parse_slot)(input);
}

/// Parse all the lines of the grid
pub fn parse_slot_grid(input: &str) -> IResult<&str, Vec<Vec<Option<Crate>>>> {
    return separated_list0(tag("\r\n"), parse_slot_line)(input);
}

pub fn parse_indices(input: &str) -> IResult<&str, Vec<&str>> {
    return preceded(tag(" "), separated_list0(tag("   "), digit1))(input);
}
pub fn parse_single_instruction(input: &str) -> IResult<&str, Instruction> {
    return map(
        tuple((
            preceded(tag("move "), parse_usize),
            preceded(tag(" from "), parse_usize),
            preceded(tag(" to "), parse_usize),
        )),
        |(move_, from, to)| Instruction { move_, from, to },
    )(input);
}

pub fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    return separated_list0(multispace1, parse_single_instruction)(input);
}

pub fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

// Tests

#[cfg(test)]
mod test {
    use crate::{parse_slot_grid, Crate, parse_instructions, Instruction, parse_indices};

    #[test]
    fn parse_grid_works() {
        let grid_input = "    [D]    \r\n[N] [C]    \r\n[Z] [M] [P]";
        let (_, grid) = parse_slot_grid(grid_input).unwrap();
        assert_eq!(grid[0], vec![None, Some(Crate("D")), None]);
        assert_eq!(
            grid[2],
            vec![Some(Crate("Z")), Some(Crate("M")), Some(Crate("P"))]
        );
    }

    #[test]
    fn parse_instructions_works(){
        let instructions_input = "move 3 from 2 to 4
move 2 from 8 to 3
move 5 from 8 to 2
move 4 from 2 to 5";
        let (_, instructions) = parse_instructions(instructions_input).unwrap();
        assert_eq!(instructions[3], Instruction{move_:4,from:2, to:5});
    }

    #[test]
    fn parse_indices_works() {
        let indices_input =" 1   2   3   4 ";
        let (_,indices) = parse_indices(indices_input).unwrap();

        assert_eq!(indices, vec!["1","2","3","4"]);
    }
}



fn main() {
    let input = include_str!(".\\real_input.txt");

    let (_, (grid, instructions)) = parse_input(input).unwrap();
    let crate_stacks = build_crate_stacks(grid); 

    // Roll part one!
    let part_one_crate_stacks = part_one_rolling(&crate_stacks, &instructions);

    let result_part_one = part_one_crate_stacks
        .iter()
        .fold(String::new(), |res, stack| format!("{}{}", res, stack.last().unwrap().0));

    dbg!(result_part_one);

    // Roll part two!

    let part_two_crate_stacks = part_two_rolling(&crate_stacks,&instructions);

    let result_part_two = part_two_crate_stacks
        .iter()
        .fold(String::new(), |res, stack| format!("{}{}", res, stack.last().unwrap().0));

    dbg!(result_part_two);


}
