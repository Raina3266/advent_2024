#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

pub fn part_1(string: &str) -> i32 {
    let mut nums: Vec<String> = string
        .split_ascii_whitespace()
        .map(|a| a.to_string())
        .collect();
    for _ in 0..24 {
        let mut n = 0;
        while n < nums.len() {
            if nums[n] == "0" {
                nums[n] = "1".to_string();
                n += 1;
            } else if nums[n].chars().count() % 2 == 0 {
                let (left, right) = nums[n].split_at(nums[n].len() / 2);
                let replace: Vec<String> = vec![
                    left.trim_start_matches('0').to_string(),
                    right.trim_start_matches('0').to_string(),
                ];
                nums.splice(n..n + 1, replace);
                println!("{nums:?}");
                n += 2;
            } else {
                let new_num = nums[n].parse::<i32>().unwrap() * 2024;
                let new = new_num.to_string();
                nums[n] = new;
                n += 1;
            }
        }
    }
    nums.len().try_into().unwrap()
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 55312);
}
