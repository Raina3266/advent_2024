#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

// here: https://adventofcode.com/2022/day/4


pub fn part_1(string: &str) -> i32 {
    let mut count = 0;
    for pair in string.lines() {
        let mut group = pair.split(',').map(|elf| {
            let mut id = elf.split('-').map(|id| id.parse::<i32>().unwrap());
            (id.next().unwrap(), id.next().unwrap())
        });
        let first = group.next().unwrap();
        let second = group.next().unwrap();
        if first.0 <= second.0 && first.1 >= second.1 || first.0 >= second.0 && first.1 <= second.1
        {
            count += 1;
        }
    }
    count
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(INPUT), 2);
}

pub fn part_2(string: &str) -> i32 {
    let mut count = 0;
    for pair in string.lines() {
        let mut group = pair.split(',').map(|elf| {
            let mut id = elf.split('-').map(|id| id.parse::<i32>().unwrap());
            (id.next().unwrap(), id.next().unwrap())
        });
        let first = group.next().unwrap();
        let second = group.next().unwrap();
        if first.0 < second.1 && first.1 >= second.0
            || second.0 < first.1 && first.0 <= second.1
            || first.0 == second.0
            || first.1 == second.1
        {
            count += 1;
        }
    }
    count
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(INPUT), 4);
}
