#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

pub fn part_1(string: &str) -> i32 {
    string
        .lines()
        .map(|elf| elf.split_at(elf.len() / 2))
        .map(|(a, b)| {
            a.chars()
                .filter(|c| b.contains(*c))
                .map(letter_to_number)
                .next()
                .unwrap()
        })
        .sum()
}

fn letter_to_number(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 'a' as i32 + 1,
        'A'..='Z' => c as i32 - 'A' as i32 + 27,
        _ => panic!("invalid character"),
    }
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 157);
}

pub fn part_2(string: &str) -> i32 {
    let elves: Vec<&str> = string.lines().collect();
    let mut sum = 0;
    for group in elves.chunks(3) {
        let same_char_in_the_first_two_bag: Vec<char> = group[0]
            .chars()
            .filter(|char| group[1].contains(*char))
            .collect();
        sum += group[2]
            .chars()
            .filter(|char| same_char_in_the_first_two_bag.contains(char))
            .map(letter_to_number)
            .next().unwrap();
    }
    sum
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(INPUT), 70);
}
