use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
    io::Result,
};

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    let mut queue = VecDeque::with_capacity(14);
    for (i, c) in (1..).zip(input.chars()) {
        if queue.len() == 14 {
            queue.pop_front();
        }
        queue.push_back(c);
        let accumulated = queue.iter().fold(HashMap::new(), |mut acc, c| {
            acc.entry(c).and_modify(|count| *count += 1).or_insert(1);
            println!("{acc:?}");
            acc
        });

        if accumulated
            .values()
            .max()
            .expect("Should be able to calculate the maximum.")
            == &1
            && queue.len() == 14
        {
            println!("{}", i - 14);
            break;
        }
    }
    Ok(())
}
