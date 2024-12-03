
#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

pub fn part_1(string: &str) -> i32 {
    let mut ans = 0;
    let mut list_of_reports: Vec<Vec<i32>> = vec![];
    for line in string.lines() {
        let report = line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|r| r.parse::<i32>().unwrap())
            .collect();
        list_of_reports.push(report);
    }

    for report in list_of_reports {
        // increase
        if report[0] < report[1] {
            if check_single_report_is_increase(&report) {
                ans += 1
            }
        // decrease
        } else if report[0] > report[1]{
            let new_report: Vec<i32> = report.into_iter().rev().collect();
            if check_single_report_is_increase(&new_report) {
                ans += 1;
            }
        } else {
            println!("skip");
            continue;
        }
    }
    ans
}

fn check_single_report_is_increase(report: &Vec<i32>) -> bool {
    for num in 1..report.len() {
        if report[num - 1] >= report[num] || report[num] - report[num - 1] > 3 {
            return false
        }
    }
    true
}


#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 2);
}




pub fn part_2(string: &str) -> i32 {
    let mut ans = 0;
    let mut list_of_reports: Vec<Vec<i32>> = vec![];
    for line in string.lines() {
        let report = line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|r| r.parse::<i32>().unwrap())
            .collect();
        list_of_reports.push(report);
    }

    for report in list_of_reports {
        if check_problem_dampener(&report) {
                ans += 1;
        } else {
            let new_report: Vec<i32> = report.into_iter().rev().collect();
            if check_problem_dampener(&new_report) {
                ans += 1;
            }
        } 
    }
    ans
}

fn check_problem_dampener(report: &Vec<i32>) -> bool {
    if !check_single_report_is_increase(report) {
        for num in 0..report.len() {
            let mut new_report = report.clone();
            new_report.remove(num);

            if check_single_report_is_increase(&new_report) {
                return true;
            }
        }
        return false;
    } 
    true
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(TEST_INPUT), 10);
}
