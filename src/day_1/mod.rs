use std::collections::HashMap;

#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

pub fn part_1(string: &str) -> i32 {
    let mut vec_left: Vec<i32> = vec![];
    let mut vec_right: Vec<i32> = vec![];

    for line in string.lines() {
        let (left, right) = line.split_once("   ").unwrap();
        vec_left.push(left.parse().unwrap());
        vec_right.push(right.parse().unwrap());
    }

    vec_left.sort_unstable();
    vec_right.sort_unstable();

    let mut ans = 0;

    for (left, right) in std::iter::zip(vec_left, vec_right) {
        let distance = (left - right).abs();
        ans += distance;
    }

    ans
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 11);
}

pub fn part_2(string: &str) -> i32 {
    let mut ans = 0;
    let mut vec_left: Vec<i32> = vec![];
    let mut vec_right: Vec<i32> = vec![];

    for line in string.lines() {
        let (left, right) = line.split_once("   ").unwrap();
        vec_left.push(left.parse().unwrap());
        vec_right.push(right.parse().unwrap());
    }

    let mut left_map: HashMap<i32, i32> = HashMap::new();
    let mut right_map: HashMap<i32, i32> = HashMap::new();

    for left in vec_left {
        *left_map.entry(left).or_insert(0) += 1
    }
    for right in vec_right {
        *right_map.entry(right).or_insert(0) += 1
    }
    
    left_map.iter().for_each(|(key, value)| ans += key * value * *right_map.get(key).unwrap_or(&0));
    ans
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(TEST_INPUT), 31);
}


