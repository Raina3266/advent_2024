#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");
type Number = u64;

type Num = u64;

pub fn part_1(string: &str) -> i64 {
    let mut nums = parse_numbers(string);

    for i in 0..25 {
        println!("print step {i}");
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

fn count_digits(mut num: Num) -> usize {
    let mut digits = 0;
    while num != 0 {
        num /= 10;
        digits += 1;
    }
    digits
}

fn split_num_into_digits(num: Num) -> (Num, Num) {
    let digits = count_digits(num);
    
}

#[test]
fn digits() {
    let num = 98327489;
   let digits = count_digits(num);
   assert_eq!(digits, 8);
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 55312);
}
