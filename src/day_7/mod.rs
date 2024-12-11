use std::{
    collections::{self, VecDeque},
    fmt::Binary,
};

#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

#[derive(PartialEq, Clone, Copy, Debug)]
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
    let mut all_operations: Vec<Vec<Operation>> = vec![];
    let mut changing_operations = vec![Operation::Add; nums.len() - 1];
    let mut index = nums.len() - 2;
    for index in (0..=nums.len() - 2).rev() {
        let mut collection = operations_for_one_line(&mut changing_operations, index);
        all_operations.append(&mut collection);
    }
    println!("{all_operations:?}");

    for operation in all_operations {
        if total == compute(&operation, nums) {
            return true;
        }
    }
    false
}

fn operations_for_one_line(operations: &mut [Operation], index: usize) -> Vec<Vec<Operation>> {
    let mut collections: Vec<Vec<Operation>> = vec![];
    let mut first = operations.len();
    let last = operations.len() - 1;
    loop {
        first -= 1;
        for i in (first..=last).rev() {
            while operations[i].change().is_some() {
                operations[i] = operations[i].change().unwrap();
                collections.push(operations.to_vec());
            }
        }

        if first == index && operations[first].change().is_none() {
            break;
        }
    }
    collections
}

fn compute(operations: &[Operation], nums: &[i32]) -> i32 {
    let mut total: i32 = nums[0];
    for (num, operation) in nums[1..].iter().zip(operations.iter()) {
        total = operation.calculate(*num, total);
    }
    total
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 292);
}
