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
    fn switch(&self) -> Option<Self> {
        match self {
            Self::Add => Some(Self::Mul),
            Self::Mul => None,
        }
    }
    fn calculation(&self, x: i32, y: i32) -> i32{
        match self {
            Self::Add => x + y,
            Self::Mul => x * y,
        }
    }
}

pub fn part_1(string: &str) -> i32 {
    for l in string.lines(){

    }
    todo!()
}

fn check_one_line(nums: Vec<i32>, result: i32) -> bool {
    let mut operations: VecDeque<Operation> = VecDeque::new();
    let length = nums.len() - 1;
}

fn operations_combination(mut operations: VecDeque<Operation>, length: usize) -> Vec<Vec<Operation>> {
    let mut result: Vec<Vec<i32>> = vec![];
    operations.push_front(Operation::Add);

    for i in 0..length{
        if operations[i] == Operation::Add {
            operations[i].switch();
        } else {
            operations[i].switch();
            operations.push_front(Operation::Mul);
        }
    }
    println!("{:?}", result);
    result
}

#[test]
fn feature() {

    
}


#[test]
fn part_1_test() {
    for my_int in 0..10{
        println!("{my_int:b}");
    }
    
    assert_eq!(operations_combination(5), [[1,2],[2,3]]);
}


