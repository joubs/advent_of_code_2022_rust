use std::collections::HashSet;

use nom::{
    character::complete::u32,
    character::{complete::anychar, streaming::space1},
    combinator::map,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug)]
pub struct Instruction {
    direction: Direction,
    steps: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point2D {
    x: i32,
    y: i32,
}

impl Point2D {
    fn is_adjacent(&self, other: &Point2D) -> bool {
        return (other.y - self.y).abs() <= 1 && (other.x - self.x).abs() <= 1;
    }
}

// Parser

pub fn parse_instruction(input_line: &str) -> IResult<&str, Instruction> {
    return map(
        separated_pair(anychar, space1, u32),
        |(cha, steps)| match cha {
            'U' => Instruction {
                direction: Direction::Up,
                steps,
            },
            'D' => Instruction {
                direction: Direction::Down,
                steps,
            },
            'L' => Instruction {
                direction: Direction::Left,
                steps,
            },
            'R' => Instruction {
                direction: Direction::Right,
                steps,
            },
            _ => panic!("Wrong input"),
        },
    )(input_line);
}

pub fn increment_pos(pos: &mut Point2D, direction: &Direction) {
    match direction {
        Direction::Left => pos.x -= 1,
        Direction::Right => pos.x += 1,
        Direction::Up => pos.y += 1,
        Direction::Down => pos.y -= 1,
    }
}

pub fn update_snake(snake: &mut [Point2D]) {
    for index in 0..(snake.len()-1) {
        let head = snake[index];
        let tail = &mut snake[index + 1];

        if !head.is_adjacent(tail) {
            let dx = head.x - tail.x;
            let dy = head.y - tail.y;
    
            match dx {
                d if d > 0 => tail.x += 1,
                d if d < 0 => tail.x -= 1,
                0 => {}
                _ => panic!()
            }
            match dy {
                d if d > 0 => tail.y += 1,
                d if d < 0 => tail.y -= 1,
                0 => {}
                _ => panic!()
            }            
        }

    }
}

fn play_snake(snake: &mut [Point2D], instructions: &Vec<Instruction>) -> u32 {
    let mut tail_pos_registry: HashSet<Point2D> = HashSet::new();
    let tail_index = snake.len() - 1;

    tail_pos_registry.insert(snake[tail_index]);
    for instruction in instructions.iter() {
        for _ in 0..instruction.steps {
            increment_pos( &mut snake[0], &instruction.direction);
            update_snake(snake);
            tail_pos_registry.insert(snake[tail_index]);
        }
    }

    return tail_pos_registry.len() as u32;
}

#[cfg(test)]
mod tests {
    use crate::{increment_pos, Direction, Point2D};

    #[test]
    fn increment_pos_works() {
        let mut init_pos = Point2D { x: 2, y: 3 };
        increment_pos(&mut init_pos, &Direction::Up);
        assert_eq!(init_pos, Point2D { x: 2, y: 4 });
        increment_pos(&mut init_pos, &Direction::Down);
        assert_eq!(init_pos, Point2D { x: 2, y: 3 });
        increment_pos(&mut init_pos, &Direction::Right);
        assert_eq!(init_pos, Point2D { x: 3, y: 3 });
        increment_pos(&mut init_pos, &Direction::Left);
        assert_eq!(init_pos, Point2D { x: 2, y: 3 });
    }

    #[test]
    fn test_adjacent_pos() {
        let p = Point2D { x: 5, y: 8 };
        assert_eq!(p.is_adjacent(&Point2D { x: 6, y: 9 }), true);
        assert_eq!(p.is_adjacent(&Point2D { x: 4, y: 9 }), true);
        assert_eq!(p.is_adjacent(&Point2D { x: 6, y: 8 }), true);
        assert_eq!(p.is_adjacent(&Point2D { x: 6, y: 10 }), false);
    }
}

fn main() {
    let instructions: Vec<Instruction> = include_str!(".\\real_input.txt")
        .lines()
        .map(|l| parse_instruction(l).unwrap().1)
        .collect();

    // dbg!(&instructions);
    let answer_part_one = play_snake(&mut [Point2D{x:0, y: 0}; 2], &instructions);
    dbg!(answer_part_one);

    let answer_part_two = play_snake(&mut [Point2D{x:0, y: 0}; 10], &instructions);
    dbg!(answer_part_two);


}
