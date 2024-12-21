#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

#[derive(PartialEq, Clone, Copy, Debug)]
enum Part_One_Operation {
    Add,
    Mul,
}

impl Part_One_Operation {
    fn next(&self) -> Option<Self> {
        match self {
            Part_One_Operation::Add => Some(Part_One_Operation::Mul),
            Part_One_Operation::Mul => None,
        }
    }
    fn calculate(&self, x: i64, y: i64) -> i64 {
        match self {
            Part_One_Operation::Add => x + y,
            Part_One_Operation::Mul => x * y,
        }
    }
}

pub fn part_1(string: &str) -> i64 {
    let mut ans = 0;
    for line in string.lines() {
        let split: Vec<&str> = line.split_ascii_whitespace().collect();
        let nums: Vec<i64> = split[1..].iter().map(|n| n.parse().unwrap()).collect();
        let total: i64 = split[0].trim_end_matches(':').parse().unwrap();
        let list_of_operations = create_operations_part_1(nums.len() - 1);

        if check_each_line_part_1(total, list_of_operations, &nums) {
            ans += total;
        }
    }
    ans
}

fn create_operations_part_1(length: usize) -> Vec<Vec<Part_One_Operation>> {
    let mut operations = vec![Part_One_Operation::Add; length];
    let mut result: Vec<Vec<Part_One_Operation>> = vec![operations.clone()];

    loop {
        create_one_operation_part_1(&mut operations);
        result.push(operations.clone());
        if operations.iter().all(|opt| *opt == Part_One_Operation::Mul) {
            return result;
        }
    }
}

fn create_one_operation_part_1(input: &mut [Part_One_Operation]) {
    let mut pointer = input.len() - 1;
    loop {
        if input[pointer].next().is_some() {
            input[pointer] = input[pointer].next().unwrap();
            break;
        } else {
            input[pointer] = Part_One_Operation::Add;
            pointer -= 1;
        }
    }
}

fn compute_part_1(operations: Vec<Part_One_Operation>, nums: &[i64]) -> i64 {
    let mut result = nums[0];
    for (a, b) in nums[1..].iter().zip(operations) {
        result = b.calculate(*a, result)
    }
    result.into()
}

fn check_each_line_part_1(
    total: i64,
    list_of_operations: Vec<Vec<Part_One_Operation>>,
    nums: &[i64],
) -> bool {
    for operations in list_of_operations {
        if compute_part_1(operations, &nums) == total {
            return true;
        }
    }
    false
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 3749);
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Part_Two_Operation {
    Add,
    Mul,
    Con,
}

impl Part_Two_Operation {
    fn next(&self) -> Option<Self> {
        match self {
            Part_Two_Operation::Add => Some(Part_Two_Operation::Mul),
            Part_Two_Operation::Mul => Some(Part_Two_Operation::Con),
            Part_Two_Operation::Con => None,
        }
    }
    fn calculate(&self, x: i64, y: i64) -> i64 {
        match self {
            Part_Two_Operation::Add => x + y,
            Part_Two_Operation::Mul => x * y,
            Part_Two_Operation::Con => concatenation(x, y),
        }
    }
}

fn concatenation(x: i64, y: i64) -> i64 {
    let mut y_length = 0;
    let mut y_copy = y;
    while y_copy != 0 {
        y_copy /= 10;
        y_length += 1;
    }
    x * 10_i64.pow(y_length) + y
}

pub fn part_2(string: &str) -> i64 {
    let mut ans = 0;
    for line in string.lines() {
        let split: Vec<&str> = line.split_ascii_whitespace().collect();
        let nums: Vec<i64> = split[1..].iter().map(|n| n.parse().unwrap()).collect();
        let total: i64 = split[0].trim_end_matches(':').parse().unwrap();
        let list_of_operations = create_operations_part_2(nums.len() - 1);

        if check_each_line_part_2(total, list_of_operations, &nums) {
            ans += total;
        }
    }
    ans
}

fn create_operations_part_2(length: usize) -> Vec<Vec<Part_Two_Operation>>{
    let mut list_of_operations: Vec<Vec<Part_Two_Operation>> = vec![];
    let mut operations = vec![Part_Two_Operation::Add; length];
    loop {
        list_of_operations.push(operations.to_vec());
        if operations.iter().all(|opt| *opt == Part_Two_Operation::Con) {
            break;
        }
        create_one_operation_part_2(&mut operations);
    }
    list_of_operations
}

fn create_one_operation_part_2(operations: &mut [Part_Two_Operation]) {
    let mut pointer = operations.len() -1;
    loop {
        if operations[pointer].next().is_some() {
            operations[pointer] = operations[pointer].next().unwrap();
            break;
        } else {
            operations[pointer] = Part_Two_Operation::Add;
            pointer -= 1;
        }
    }

}

fn check_each_line_part_2(
    total: i64,
    list_of_operations: Vec<Vec<Part_Two_Operation>>,
    nums: &[i64],
) -> bool {
    for operations in list_of_operations {
        if compute_part_2(operations.clone(), nums) == total {
            return true;
        }
    }
    false
}

fn compute_part_2(operations: Vec<Part_Two_Operation>, nums: &[i64]) -> i64 {
    let mut result = nums[0];
    for (a, b) in nums[1..].iter().zip(operations) {
        result = b.calculate(result, *a);
    }
    result
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(TEST_INPUT), 11387);
}
