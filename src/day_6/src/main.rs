use itertools::Itertools;
fn main() {
    let input = include_bytes!(".\\real_input.txt");

    let part_one_answer = input
        .windows(4)        
        .enumerate()
        .find_or_first(|(_, tup)| tup.iter().unique().collect_vec().len() == tup.len())
        .unwrap().0 + 4;
    
    dbg!(part_one_answer);

    let part_two_answer = input
    .windows(14)        
    .enumerate()
    .find_or_first(|(_, tup)| tup.iter().unique().collect_vec().len() == tup.len())
    .unwrap().0 + 14;

dbg!(part_two_answer);


}
