use std::str::FromStr;

#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn score(self) -> i32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err("must be A,B,C,X,Y,Z".to_string()),
        }
    }
}

impl Shape {
    fn score_for_shape(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
    fn outcome(enemy_shape: &Self, my_shape: Shape) -> Outcome {
        match enemy_shape {
            Shape::Rock => match my_shape {
                Shape::Rock => Outcome::Draw,
                Shape::Paper => Outcome::Win,
                Shape::Scissors => Outcome::Lose,
            },
            Shape::Paper => match my_shape {
                Shape::Rock => Outcome::Lose,
                Shape::Paper => Outcome::Draw,
                Shape::Scissors => Outcome::Win,
            },
            Shape::Scissors => match my_shape {
                Shape::Rock => Outcome::Win,
                Shape::Paper => Outcome::Lose,
                Shape::Scissors => Outcome::Draw,
            },
        }
    }
}

// inherent impl: impl Shape {}
// trait impl: impl FromStr for Shape {}

pub fn part_1(string: &str) -> i32 {
    let mut total_score = 0;
    for line in string.lines() {
        let mut iter = line.split(' ');
        let enemy_shape: Shape = iter.next().unwrap().parse().unwrap();
        let my_shape: Shape = iter.next().unwrap().parse().unwrap();
        total_score += my_shape.score_for_shape() + Shape::outcome(&enemy_shape, my_shape).score();
    }
    total_score
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 15);
}

fn shape_to_choose(first: &str, result: &str) -> i32 {
    match result {
        "X" => match first {
            "A" => 3,
            "B" => 1,
            "C" => 2,
            _ => unreachable!(),
        },
        "Y" => match first {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => unreachable!(),
        },
        "Z" => match first {
            "A" => 2,
            "B" => 3,
            "C" => 1,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

pub fn part_2(string: &str) -> i32 {
    let mut total_score = 0;
    for line in string.lines() {
        let mut iter = line.split(' ');
        let round: (&str, &str) = (iter.next().unwrap(), iter.next().unwrap());
        let point = match round.1 {
            "X" => shape_to_choose(round.0, "X"),
            "Y" => shape_to_choose(round.0, "Y") + 3,
            "Z" => shape_to_choose(round.0, "Z") + 6,
            _ => unreachable!(),
        };
        total_score += point;
    }
    total_score
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(INPUT), 12);
}
