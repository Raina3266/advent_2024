#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

pub fn part_1(string: &str) -> i32 {
    let mut first_section: Vec<(i32, i32)> = vec![];
    let mut second_section: Vec<Vec<i32>> = vec![];

    let (first_part, second_part) = string.split_once("\n\n").unwrap();
    for f in first_part.lines() {
        let (a, b) = f.split_once("|").unwrap();
        first_section.push((a.parse().unwrap(), b.parse().unwrap()));
    }
    for s in second_part.lines() {
        let updates: Vec<i32> = s.split(",").map(|u| u.parse::<i32>().unwrap()).collect();
        second_section.push(updates);
    }

    let mut correct_pair: Vec<i32> = vec![];
    for update in second_section {
        let pairs = pair_two_indice_for_one_update(&update);
        if check_safety(&first_section, &update, pairs) {
            correct_pair.push(update[update.len() / 2]);
        }
    }
    correct_pair.iter().sum()
}

fn pair_two_indice_for_one_update(update: &[i32]) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = vec![];
    for first_index in 0..(update.len() - 1) {
        let mut second_index: usize = first_index + 1;
        while second_index < update.len() {
            result.push((first_index, second_index));
            second_index += 1
        }
    }
    result
}

fn check_safety(list: &[(i32, i32)], update: &[i32], pairs: Vec<(usize, usize)>) -> bool {
    for pair in pairs {
        if list.iter().all(|p| *p != (update[pair.0], update[pair.1])) {
            return false;
        }
    }
    true
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 143);
}

pub fn part_2(string: &str) -> i32 {
    let mut first_section: Vec<(i32, i32)> = vec![];
    let mut second_section: Vec<Vec<i32>> = vec![];

    let (first_part, second_part) = string.split_once("\n\n").unwrap();
    for f in first_part.lines() {
        let (a, b) = f.split_once("|").unwrap();
        first_section.push((a.parse().unwrap(), b.parse().unwrap()));
    }
    for s in second_part.lines() {
        let updates: Vec<i32> = s.split(",").map(|u| u.parse::<i32>().unwrap()).collect();
        second_section.push(updates);
    }

    let mut incorrect_pairs: Vec<Vec<i32>> = vec![];
    for update in second_section {
        let pairs = pair_two_indice_for_one_update(&update);
        if !check_safety(&first_section, &update, pairs) {
            incorrect_pairs.push(update);
        }
    }

    let mut fixed: Vec<i32> = vec![];
    for to_fixed in incorrect_pairs {
        let fixed_order = fix_incorrect_pair(to_fixed, &first_section);
        fixed.push(fixed_order[fixed_order.len() / 2]);
    }
    fixed.iter().sum()
}

fn fix_incorrect_pair(mut wrong: Vec<i32>, list: &[(i32, i32)]) -> Vec<i32> {
    let pairs = pair_two_indice_for_one_update(&wrong);
    for pair in pairs {
        if list.iter().all(|p| *p != (wrong[pair.0], wrong[pair.1])) {
            wrong.swap(pair.0, pair.1);
        }
    }
    wrong
}


#[test]
fn part_2_test() {
    assert_eq!(part_2(TEST_INPUT), 123);
}






#[test]
fn feature() {
    let nums = vec![97, 13, 75, 29, 47];
    let list = vec![
        (47, 53),
        (97, 13),
        (97, 61),
        (97, 47),
        (75, 29),
        (61, 13),
        (75, 53),
        (29, 13),
        (97, 29),
        (53, 29),
        (61, 53),
        (97, 53),
        (61, 29),
        (47, 13),
        (75, 47),
        (97, 75),
        (47, 61),
        (75, 61),
        (47, 29),
        (75, 13),
        (53, 13),
    ];
    assert_eq!(fix_incorrect_pair(nums, &list), vec![97,75,47,29,13])
}
