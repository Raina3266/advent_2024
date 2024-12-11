use std::{collections::VecDeque, fmt::Binary};

#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

#[derive(PartialEq, Clone, Copy)]
enum Operation {
    Add,
    Mul,
}

impl Operation {
    fn change(&self) -> Option<Self> {
        match self {
            Self::Add => Some(Self::Mul),
            Self::Mul => None,
        }
    }
    fn calculate(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Mul => x * y,
        }
    }
}

pub fn part_1(string: &str) -> i32 {
    let mut ans = 0;
    for line in string.lines() {
        let (first_part, second_part) = line.split_once(": ").unwrap();
        let total: i32 = first_part.parse().unwrap();
        let nums: Vec<i32> = second_part
            .split_ascii_whitespace()
            .map(|a| a.parse().unwrap())
            .collect();
        if check_each_line(&nums, total) {
            ans += total;
        }
    }
    ans
}

fn check_each_line(nums: &[i32], total: i32) -> bool {
    let mut initial_operations = vec![Operation::Add; nums.len() - 1];
    let all_operations = operations_for_one_line(&mut initial_operations);
    for operation in all_operations {
        let start_num: i32 = nums[0];
        let result: i32 = nums[1..operation.len()]
            .iter()
            .zip(operation.iter())
            .fold(start_num, |acc, (num, op)| op.calculate(acc, *num));
        if result == total {
            return true;
        }
    }
    false
}

fn operations_for_one_line(operations: &mut [Operation]) -> Vec<Vec<Operation>> {
    let mut result: Vec<Vec<Operation>> = vec![];
    result.push(operations.to_vec());
    let mut position = operations.len() - 1;
    loop {
        for index in (position..operations.len() - 1).rev() {
            if operations[index].change().is_some() {
                operations[index] = operations[index].change().unwrap();
                result.push(operations.to_vec());
            }
        }
        if position == 0 {
            break;
        }
        position -= 1;
    }
    result
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 3749);
}
