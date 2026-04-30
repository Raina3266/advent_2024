use std::collections::HashMap;

#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

#[derive(PartialEq, Clone, Debug)]
struct Grid {
    data: Vec<Vec<Node>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let mut data: Vec<Vec<Node>> = Vec::new();
        let mut height = 0;
        for line in input.lines().rev() {
            height += 1;
            data.push(
                line.chars()
                    .map(|step| match step {
                        'O' => Node::Box,
                        '#' => Node::Wall,
                        '.' => Node::Empty,
                        _ => unreachable!(),
                    })
                    .collect(),
            )
        }
        let width = data[0].len();
        Grid {
            data,
            width,
            height,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Node {
    Empty,
    Box,
    Wall,
}

#[derive(Clone, Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn new(input: &str) -> Vec<Move> {
        input
            .chars()
            .map(|step| match step {
                '^' => Move::Up,
                'v' => Move::Down,
                '>' => Move::Right,
                '<' => Move::Left,
                _ => unreachable!(),
            })
            .collect()
    }
}

pub fn part_1(string: &str) -> i32 {
    let mut iter = string.split("\n\n");
    let grid = Grid::new(iter.next().unwrap());
    let moves = Move::new(iter.next().unwrap());
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 104);
}

pub fn part_2(string: &str) -> i32 {
    todo!()
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(TEST_INPUT), 31);
}
