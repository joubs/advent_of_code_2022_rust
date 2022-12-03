
pub fn compute_score_part_one(game: &str) -> i64 {

    match game {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3 ,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => panic!("Wrong input!")
    }
}

pub fn compute_score_part_two(game: &str) -> i64 {

    match game {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        _ => panic!("Wrong input!")
    }
}

fn main() -> anyhow::Result<()> {

    let result_part_one: i64 = include_str!("real_input.txt")
        .split("\r\n")
        .map(compute_score_part_one)
        .sum();
    
    dbg!(result_part_one);

    let result_part_two: i64 = include_str!("real_input.txt")
        .split("\r\n")
        .map(compute_score_part_two)
        .sum();
    
    dbg!(result_part_two);
    Ok(())
}
