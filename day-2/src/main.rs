use std::{fs::read_to_string, io};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl TryFrom<&str> for Hand {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(format!("Unknown symbol for Hand: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
enum Outcome {
    Win = 6,
    Loss = 0,
    Draw = 3,
}

impl TryFrom<&str> for Outcome {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(format!("Unknown symbol for Outcome: {}", s)),
        }
    }
}

impl Outcome {
    fn achieve_against(&self, hand: &Hand) -> Hand {
        match (self, hand) {
            (Outcome::Loss, Hand::Rock) => Hand::Scissors,
            (Outcome::Win, Hand::Rock) => Hand::Paper,
            (Outcome::Loss, Hand::Paper) => Hand::Rock,
            (Outcome::Win, Hand::Paper) => Hand::Scissors,
            (Outcome::Loss, Hand::Scissors) => Hand::Paper,
            (Outcome::Win, Hand::Scissors) => Hand::Rock,
            (Outcome::Draw, hand) => *hand,
        }
    }
}

impl Hand {
    fn play(&self, other: &Self) -> Outcome {
        match (self, other) {
            (Hand::Rock, Hand::Scissors) => Outcome::Win,
            (Hand::Rock, Hand::Paper) => Outcome::Loss,
            (Hand::Scissors, Hand::Paper) => Outcome::Win,
            (Hand::Scissors, Hand::Rock) => Outcome::Loss,
            (Hand::Paper, Hand::Rock) => Outcome::Win,
            (Hand::Paper, Hand::Scissors) => Outcome::Loss,
            _ => Outcome::Draw,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Match {
    elf: Hand,
    me: Hand,
    outcome: Outcome,
    score: u32,
}

impl Match {
    fn from_line(line: &str) -> Self {
        let (elf, outcome) = line.split_once(" ").expect("Failed to split on space.");
        let elf = elf.try_into().expect("Couldn't convert to Hand");
        let outcome: Outcome = outcome.try_into().expect("Couldn't convert to Outcome");
        let me = outcome.achieve_against(&elf);
        let score = outcome.clone() as u32 + me.clone() as u32;
        Match {
            elf,
            me,
            outcome,
            score,
        }
    }
}

fn main() -> io::Result<()> {
    let input = read_to_string("input.txt")?;
    let score: u32 = input
        .lines()
        .map(Match::from_line)
        .map(|x| {
            println!("{x:?}");
            x.score
        })
        .sum();
    println!("Your score is: {}", score);
    Ok(())
}
