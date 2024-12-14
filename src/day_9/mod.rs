#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

pub fn part_1(string: &str) -> i32 {
    let mut nums: Vec<String> = string
        .char_indices()
        .map(|(i, _)| string[i..i + 1].to_string())
        .collect();
    println!("{nums:?}");
    let mut id = 0;
    for index in 0..nums.len() {
        if index % 2 == 0 {
            let replace = vec![id.to_string(); nums[index].parse::<usize>().unwrap()];
            nums.splice(index..replace.len(), replace);
            id += 1;
        } else {
            let replace = vec![String::from("."); nums[index].parse::<usize>().unwrap()];
            nums.splice(index..replace.len(), replace);
        }
    }
    println!("{nums:?}");
    todo!()
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 1928);
}
