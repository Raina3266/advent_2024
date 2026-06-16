#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

struct Ship {
    crates: Vec<Vec<char>>,
}

impl Ship {
    fn number_of_stacks(line: &str) -> usize {
        (line.chars().count() + 1) / 4
    }

    fn new(input: &str) -> Ship {
        let mut crates: Vec<Vec<char>> =
            vec![vec![]; Ship::number_of_stacks(input.lines().next().unwrap())];
        for line in input.lines().rev().skip(1) {
            for n in 0..Ship::number_of_stacks(line) {
                let chars: Vec<char> = line.chars().collect();
                if !chars[4 * n + 1].is_ascii_whitespace() {
                    crates[n].push(chars[4 * n + 1]);
                }
            }
        }
        Ship { crates }
    }
}

#[derive(Debug)]
struct Step {
    moves: usize,
    move_from: usize,
    move_to: usize,
}

impl Step {
    fn new(input: &str) -> Vec<Step> {
        let mut steps = Vec::new();
        for line in input.lines() {
            let iter: Vec<&str> = line.split(' ').collect();

            steps.push(Step {
                moves: iter[1].parse::<usize>().unwrap(),
                move_from: iter[3].parse::<usize>().unwrap(),
                move_to: iter[5].parse::<usize>().unwrap(),
            });
        }
        steps
    }
}

pub fn part_1(string: &str) -> Vec<char> {
    let mut iter = string.split("\n\n");
    let mut crates = Ship::new(iter.next().unwrap()).crates;
    let steps = Step::new(iter.next().unwrap());

    for step in steps {
        for _ in 0..step.moves {
            let pop = crates[step.move_from - 1].pop().unwrap();
            crates[step.move_to - 1].push(pop);
        }
    }
    let mut ans = vec![];
    for mut result in crates {
        ans.push(result.pop().unwrap())
    }
    ans
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(INPUT), vec!['C', 'M', 'Z']);
}

pub fn part_2(string: &str) -> Vec<char> {
    let mut iter = string.split("\n\n");
    let mut crates = Ship::new(iter.next().unwrap()).crates;
    let steps = Step::new(iter.next().unwrap());

    for step in steps {
        let mut crates_to_move = vec![];
        for _ in 0..step.moves {
            let pop = crates[step.move_from - 1].pop().unwrap();
            crates_to_move.insert(0, pop);
        }
        crates[step.move_to - 1].extend(crates_to_move);
    }
    let mut ans = vec![];
    for mut result in crates {
        ans.push(result.pop().unwrap())
    }
    ans
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(INPUT), vec!['M', 'C', 'D']);
}
