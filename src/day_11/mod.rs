#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

type Num = u64;

pub fn part_1(string: &str) -> i64 {
    let mut nums = parse_numbers(string);

    for _ in 0..75 {
        let mut n = 0;

        while n < nums.len() {
            if nums[n] == 0 {
                nums[n] = 1;
                n += 1;
                continue;
            } else if count_digits(nums[n]) % 2 == 0 {
                let (left, right) = split_num_into_digits(nums[n]);
                nums[n] = left;
                nums.insert(n + 1, right);
                n += 2;
                continue;
            } else {
                nums[n] *= 2024;
                n += 1;
            }
        }
    }
    nums.len().try_into().unwrap()
}

fn parse_numbers(input: &str) -> Vec<Num> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn count_digits(num: Num) -> usize {
    let mut digits = 0;
}

fn split_num_into_digits(num: Num) -> (Num, Num) {
    todo!()
}

#[test]
fn digits() {
    assert_eq!(0, 0000);
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 55312);
}
