use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    io::Result,
};

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() -> Result<()> {
    let low = ALPHABET.chars();
    let high = ALPHABET.to_uppercase();
    let priorities: HashMap<_, _> = low.chain(high.chars()).zip(1..).collect();

    let input = read_to_string("input.txt")?;
    let p1_result = input
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            let first: HashSet<char> = first.chars().collect();
            let second: HashSet<char> = second.chars().collect();
            let common: Vec<&char> = first.intersection(&second).collect();
            assert!(common.len() == 1);
            // Safe as we know its at least and at most of len 1
            *(priorities.get(common[0]).expect(&format!(
                "Common letter: {} wasn't in the priority mapping.",
                common[0],
            )))
        })
        .sum::<i32>();
    println!("Part #1: {p1_result}");

    let vec_lines: Vec<_> = input.lines().collect();
    let p2_result = vec_lines
        .chunks(3)
        .map(|group| {
            let common: Vec<_> = group
                .iter()
                .map(|line| line.chars().collect())
                .reduce(|one: HashSet<_>, two: HashSet<_>| {
                    one.intersection(&two).map(|c| *c).collect()
                })
                .expect("Reduction failed.")
                .drain()
                .collect();

            assert!(common.len() == 1);
            *(priorities.get(&common[0]).expect(&format!(
                "Common letter: {} wasn't in the priority mapping.",
                common[0],
            )))
        })
        .sum::<i32>();
    println!("Part #2: {p2_result}");
    Ok(())
}
