use nom::{IResult, bytes::complete::tag, character::{complete::{i64}}, branch::alt,combinator::{map, value}, sequence::preceded};



#[derive(Debug, Clone)]
pub enum Instruction {
    Noop,
    Addx(i64),
}

pub fn parse_line(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Noop, tag("noop")),
        map(preceded(tag("addx "), i64), Instruction::Addx)
    ))(input)
}

pub fn build_cpu_table(instructions: &Vec<Instruction>) -> Vec<i64>{
    let table_size = 
        instructions
            .iter()
            .map(|i| match i {
                Instruction::Noop => 1,
                Instruction::Addx(_) => 2,
            })
            .sum::<usize>();
    
    let mut cpu_table = Vec::with_capacity(table_size);

    let mut register_value = 1_i64; 
    cpu_table.push(register_value);
    for instruction in instructions {
        match instruction {
            Instruction::Noop => cpu_table.push(register_value),
            Instruction::Addx(val) => {
                cpu_table.push(register_value);
                register_value += *val;
                cpu_table.push(register_value);
            }
        }
    }
    cpu_table
}


fn main() {
    let instructions: Vec<Instruction> = include_str!(".\\real_input.txt")
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect();
    
    // dbg!(&instructions);

    let cpu_table = build_cpu_table(&instructions);
    //dbg!(&cpu_table);

    let answer_part_one = 
        vec![20_u32, 60, 100, 140, 180, 220]
            .iter()
            .map(|&i| cpu_table[(i - 1) as usize] * (i as i64))
            .sum::<i64>();
    dbg!(answer_part_one);

    let mut answer_part_two = String::from("");
    for i in 0..6 {
        for j in 0..40 {
            let sprite_pos = cpu_table[40*i+j]; 

            if [sprite_pos -1, sprite_pos, sprite_pos + 1].iter().any(|&val| val == (j as i64)){
                answer_part_two.push('#');
            } 
            else {
                answer_part_two.push('.');
            }
        }
        answer_part_two.push_str("\r\n");
    }

    println!("{}",answer_part_two);
}
