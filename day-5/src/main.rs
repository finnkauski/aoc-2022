use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
    io,
};

const INITIAL: &str = "TVJWNRMS\nVCPQJDWB\nPRDHFJB\nDNMBPRF\nBTPRVH\nTPBC\nLPRJB\nWBZTLSCN\nGSL";

type Loc = usize;

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    from: Loc,
    to: Loc,
}

impl Instruction {
    fn from_line(line: &str) -> Self {
        let elements: Vec<Option<usize>> = line
            .split_whitespace()
            .map(|element| element.parse().ok())
            .collect();
        Instruction {
            quantity: elements[1].expect("Failed to parse number"),
            from: elements[3].expect("Failed to parse number"),
            to: elements[5].expect("Failed to parse number"),
        }
    }
}

fn main() -> io::Result<()> {
    let input = read_to_string("input.txt")?;
    let instructions: Vec<Instruction> = input.lines().map(Instruction::from_line).collect();
    let mut columns: HashMap<usize, VecDeque<char>> = (1..)
        .zip(INITIAL.lines().map(|s| s.chars().collect()))
        .collect();

    for instruction in instructions {
        let from = columns
            .get_mut(&instruction.from)
            .expect("Column not found in to");

        let mut to_move: Vec<char> = Vec::with_capacity(instruction.quantity);
        for _ in 0..instruction.quantity {
            to_move.push(from.pop_front().expect("Failed to pop from"))
        }
        to_move.reverse();
        drop(from);
        let to = columns.get_mut(&instruction.to).expect("Column not found");
        for value in to_move {
            to.push_front(value)
        }
        drop(to);
    }
    let mut items: Vec<(_, _)> = columns.iter().collect();
    items.sort_by_key(|(k, _)| *k);

    println!("Part #2");
    for (_, v) in items {
        print!("{}", v[0])
    }
    Ok(())
}
