use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    //let input_str = std::fs::read_to_string("../input.txt").split("\r\n");

    // We could read the file at runtime but it is more convenient to add the 
    // file to the exe at compile time here.
    let result_part_one = include_str!("real_input.txt")
        .split("\r\n\r\n")
        .map(|group| {
            group
                .split("\r\n")
                .map(str::parse::<i64>)
                .map(Result::unwrap)
                .sum::<i64>()
        })
        .max()
        .unwrap();
    dbg!(result_part_one);

    let result_part_two = include_str!("real_input.txt")
        .split("\r\n\r\n")
        .map(|group| {
            group
                .split("\r\n")
                .map(str::parse::<i64>)
                .map(Result::unwrap)
                .sum::<i64>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum::<i64>();
    dbg!(result_part_two);

    Ok(())
}
