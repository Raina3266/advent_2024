#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

#[derive(PartialEq, Clone, Copy)]
enum Operation {
    Add,
    Mul,
}

impl Operation {
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
    let mut operations: Vec<Operation> = vec![Operation::Mul; nums.len() - 1];
    operations.iter().pe

}

fn operations_combination(mut operations: Vec<Operation>) -> Vec<Vec<Operation>> {
    let mut result: Vec<Vec<Operation>> = vec![];
    for b in 0..=255 {
        b.into()
    }
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 41);
}