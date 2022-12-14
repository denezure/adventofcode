use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Copy, Clone)]
enum RpsMove {
    Rock,
    Paper,
    Scissors,
}

impl Display for RpsMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            RpsMove::Rock => "Rock",
            RpsMove::Paper => "Paper",
            RpsMove::Scissors => "Scissors",
        })
    }
}

impl RpsMove {
    fn move_score(&self) -> usize {
        match self {
            RpsMove::Rock => 1,
            RpsMove::Paper => 2,
            RpsMove::Scissors => 3,
        }
    }

    /// What move should be made to defeat this move?
    fn get_winning_move(&self) -> Self {
        match self {
            RpsMove::Rock => RpsMove::Paper,
            RpsMove::Paper => RpsMove::Scissors,
            RpsMove::Scissors => RpsMove::Rock,
        }
    }

    /// What move should be made to lose to this move?
    fn get_losing_move(&self) -> Self {
        match self {
            RpsMove::Rock => RpsMove::Scissors,
            RpsMove::Paper => RpsMove::Rock,
            RpsMove::Scissors => RpsMove::Paper,
        }
    }
}

trait ParsingStrategy {
    fn parse_line(&mut self, line: &str) -> RpsRound;
}

#[derive(Default)]
struct PartOneStrategy {}

impl PartOneStrategy {
    fn new() -> Box<dyn ParsingStrategy> {
        Box::new(Self::default())
    }

    fn parse_move(&self, move_str: &str) -> RpsMove {
        let c = move_str.chars().next().unwrap();

        match c {
            'A' | 'X' => RpsMove::Rock,
            'B' | 'Y' => RpsMove::Paper,
            'C' | 'Z' => RpsMove::Scissors,
            _ => panic!(),
        }
    }
}

impl ParsingStrategy for PartOneStrategy {
    fn parse_line(&mut self, line: &str) -> RpsRound {
        let moves = line.split_ascii_whitespace().take(2).collect::<Vec<_>>();
        let (theirs, mine) = match moves[..] {
            [theirs, mine] => (theirs, mine),
            _ => panic!(),
        };

        let theirs = self.parse_move(theirs);
        let mine = self.parse_move(mine);

        RpsRound { theirs, mine }
    }
}

#[derive(Default)]
struct PartTwoStrategy {}

impl PartTwoStrategy {
    fn new() -> Box<dyn ParsingStrategy> {
        Box::new(Self::default())
    }

    fn parse_their_move(move_str: &str) -> RpsMove {
        let c = move_str.chars().next().unwrap();

        match c {
            'A' => RpsMove::Rock,
            'B' => RpsMove::Paper,
            'C' => RpsMove::Scissors,
            _ => panic!(),
        }
    }

    fn parse_my_move(their_move: RpsMove, move_str: &str) -> RpsMove {
        let c = move_str.chars().next().unwrap();

        match c {
            'X' => their_move.get_losing_move(),
            'Y' => their_move,
            'Z' => their_move.get_winning_move(),
            _ => panic!(),
        }
    }
}

impl ParsingStrategy for PartTwoStrategy {
    fn parse_line(&mut self, line: &str) -> RpsRound {
        let moves = line.split_ascii_whitespace().take(2).collect::<Vec<_>>();
        let (theirs, mine) = match moves[..] {
            [theirs, mine] => (theirs, mine),
            _ => panic!(),
        };

        let theirs = Self::parse_their_move(theirs);
        let mine = Self::parse_my_move(theirs, mine);

        RpsRound { theirs, mine }
    }
}

#[derive(Copy, Clone)]
struct RpsRound {
    theirs: RpsMove,
    mine: RpsMove,
}

impl RpsRound {
    fn parse_using_strategy(line: &str, strategy: &mut dyn ParsingStrategy) -> Self {
        strategy.parse_line(line)
    }

    fn score(&self) -> usize {
        self.mine.move_score() + self.outcome_score()
    }

    fn outcome_score(&self) -> usize {
        match (self.mine, self.theirs) {
            (RpsMove::Rock, RpsMove::Paper) => 0,
            (RpsMove::Paper, RpsMove::Scissors) => 0,
            (RpsMove::Scissors, RpsMove::Rock) => 0,
            (RpsMove::Rock, RpsMove::Rock) => 3,
            (RpsMove::Paper, RpsMove::Paper) => 3,
            (RpsMove::Scissors, RpsMove::Scissors) => 3,
            (RpsMove::Rock, RpsMove::Scissors) => 6,
            (RpsMove::Paper, RpsMove::Rock) => 6,
            (RpsMove::Scissors, RpsMove::Paper) => 6,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let raw_input: Vec<_> = BufReader::new(File::open("./inputs/2022/02.txt")?)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let part_one_score: usize = raw_input
        .iter()
        .map(|l| RpsRound::parse_using_strategy(&l, PartOneStrategy::new().as_mut()))
        .map(|r| r.score())
        .sum();

    println!("Part 1 result: {}", part_one_score);

    let part_two_score: usize = raw_input
        .iter()
        .map(|l| RpsRound::parse_using_strategy(&l, PartTwoStrategy::new().as_mut()))
        .map(|r| r.score())
        .sum();

    println!("Part 2 result: {}", part_two_score);

    Ok(())
}
