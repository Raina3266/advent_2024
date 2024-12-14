use std::collections::HashMap;

#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");
type Number = u64;

type Num = u64;

pub fn part_1(string: &str) -> usize {
    let nums = parse_numbers(string);

    solve_list(&nums, 75)
}

fn solve_list(nums: &[Num], blink_count: usize) -> usize {
    nums.iter()
        .copied()
        .map(|num| solve(num, blink_count))
        .sum()
}

fn solve(num: Num, blink_count: usize) -> usize {
    // The cache contains "return values from `solve` that we have already calcualated"
    //
    // For example.
    let mut cache = HashMap::new();
    solve_with_cache(num, blink_count, &mut cache)
}

fn solve_with_cache(
    num: Num,
    blink_count: usize,
    cache: &mut HashMap<(Num, usize), usize>,
) -> usize {
    let key = (num, blink_count);

    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    let count = if blink_count == 0 {
        1
    } else if num == 0 {
        solve_with_cache(1, blink_count - 1, cache)
    } else if count_digits(num) % 2 == 0 {
        let (left, right) = split_num_into_digits(num);
        solve_with_cache(left, blink_count - 1, cache)
            + solve_with_cache(right, blink_count - 1, cache)
    } else {
        solve_with_cache(num * 2024, blink_count - 1, cache)
    };

    cache.insert(key, count);
    count
}

fn blink(nums: &mut Vec<Num>) {
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
    let base: Num = (10 as Num).pow(digits as u32 / 2);

    let left = num / base;
    let right = num % base;

    (left, right)
}

#[test]
fn digits() {
    assert_eq!(count_digits(1), 1);
    assert_eq!(count_digits(123), 3);
    assert_eq!(count_digits(98327489), 8);
}

#[test]
fn split_num_works() {
    assert_eq!(split_num_into_digits(1234), (12, 34));
    assert_eq!(split_num_into_digits(11), (1, 1));
}

#[test]
fn blink_test() {
    let mut nums = vec![1];
    for _ in 0..10 {
        println!("{nums:?}");
        blink(&mut nums);
    }
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 55312);
}
