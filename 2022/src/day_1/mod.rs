#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

pub fn part_1(string: &str) -> i32 {
    string
        .split("\n\n")
        .map(|e| {
            e.split("\n")
                .map(|ee| ee.parse::<i32>().unwrap())
                .sum()
        })
        .max()
        .unwrap()

    // let mut food: Vec<&str> = string.lines().collect();
    // food.push("");
    // let mut weights: Vec<i32> = vec![];
    // let mut current_weight = 0;

}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 24000);
}

pub fn part_2(string: &str) -> i32 {
    let mut weights = string
        .split("\n\n")
        .map(|e| {
            e.split("\n")
                .map(|ee| ee.parse::<i32>().unwrap())
                .sum()
        }).collect::<Vec<i32>>();
    weights.sort();
    weights.iter().rev().take(3).sum()
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(INPUT), 45000);
}
