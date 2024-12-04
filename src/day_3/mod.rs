
#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

pub fn part_1(string: &str) -> i32 {
    let segments: Vec<&str> = string.split("mul(").collect();
    let mut collection: Vec<(i32, i32)> = vec![];
    for s in segments {
        let new_s: Vec<char> = s.chars().take(8).collect();
        if contains_sequence(&new_s, &[',',')']) {
            let new_segment: String = new_s.iter().collect();
            let new_segments: Vec<i32> = new_segment.split(&[',',')']).filter_map(|small| small.parse().ok()).collect();
            if new_segments.len() > 1 && new_segments[0] < 1000 && new_segments[1] < 1000 {
                collection.push((new_segments[0], new_segments[1]));
            }
        }

    }
    collection.iter().map(|(a, b)| a * b).sum()
}

fn contains_sequence(vec: &Vec<char>, seq: &[char]) -> bool {
    let mut seq_iter = seq.iter();
    let mut current = seq_iter.next();

    for &c in vec {
        if Some(&c) == current {
            current = seq_iter.next(); 
        }
        if current.is_none() {
            return true; 
        }
    }
    false
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 161);
}


pub fn part_2(string: &str) -> i32 {
    let indice_1: Vec<usize> = string.match_indices("don't()").map(|(i, _)|i).collect();
    let indice_2: Vec<usize> = string.match_indices("do()").map(|(i, _)|i).collect();
    let mut indices: Vec<usize> = [indice_1, indice_2].concat();
    indices.sort();

    let mut valid_segments: Vec<&str> = vec![];
    let mut start = 0;
    for index in indices {
        valid_segments.push(&string[start..index]);
        start = index;
    }
    valid_segments.push(&string[start..]);
    valid_segments.retain(|s| !s.starts_with("don't"));
    let mut ans = 0;
    for ss in valid_segments {
        ans += part_1(ss);
    }
    ans
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(TEST_INPUT), 48);
}
