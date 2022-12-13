use std::{fs::read_to_string, io::Result};

type MinMax = (u32, u32);

#[derive(Debug)]
struct Pair {
    first: MinMax,
    second: MinMax,
}

impl Pair {
    fn from_line(line: &str) -> Self {
        let (first, second) = line
            .split_once(",")
            .expect("Lines should be comma separated.");
        let min_max = |text: &str| -> MinMax {
            let (min, max) = text.split_once("-").expect("Ranges should be seperated");
            (
                min.parse().expect("Failed num parsing"),
                max.parse().expect("Failed num parsing"),
            )
        };
        Pair {
            first: min_max(first),
            second: min_max(second),
        }
    }
    fn contain_all(&self) -> bool {
        let (first_min, first_max) = self.first;
        let (second_min, second_max) = self.second;
        (first_min >= second_min && first_max <= second_max)
            || (second_min >= first_min && second_max <= first_max)
    }
    fn any_overlap(&self) -> bool {
        let (first_min, first_max) = self.first;
        let (second_min, second_max) = self.second;
        (first_max >= second_min) && (first_min <= second_max)
    }
}

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    let p1_count: u32 = input
        .lines()
        .map(|line| Pair::from_line(line).contain_all() as u32)
        .sum();
    println!("Part #1 {:?}", p1_count);
    let p2_count: u32 = input
        .lines()
        .map(|line| Pair::from_line(line).any_overlap() as u32)
        .sum();
    println!("Part #2 {:?}", p2_count);
    Ok(())
}
