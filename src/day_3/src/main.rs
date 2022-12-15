pub fn priority(cha: char) -> u32 {
    match cha {
        ('a'..='z') => 1 + (cha as u8 - b'a') as u32,
        ('A'..='Z') => 27 + (cha as u8 - b'A') as u32,
        _ => panic!(),
    }
    
}

fn main() {
    let input_lines = include_str!(".\\real_input.txt").lines();
    
    let mut answer_part_one = 0;

    for line in input_lines {
        let (first_half, second_half) = line.split_at(line.len() / 2);

        // find the intruder
        let intruder = second_half
            .chars()
            .find(|c| first_half.contains(&c.to_string()))
            .unwrap();
        
        answer_part_one += priority(intruder);
    }

    dbg!(answer_part_one);

    let input_lines = include_str!(".\\real_input.txt").lines();
    let answer_part_two = input_lines
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunks| {
            let first = chunks[0];
            let second = chunks[1];
            let third = chunks[2];

            return first.chars().find(|c| second.contains(&c.to_string()) && third.contains(&c.to_string()))
        })
        .map(|u| priority(u.unwrap()))
        .sum::<u32>();
    
    dbg!(answer_part_two);
    

}
