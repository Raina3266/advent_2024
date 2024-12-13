#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

pub fn part_1(string: &str) -> i32 {
    let raw_machines: Vec<&str> = string.split("\n\n").collect();
    let mut machines: Vec<Vec<i32>> = vec![];
    for machine in raw_machines {
        let mut nums = vec![];
        let mut char = String::new();
        for c in machine.chars() {
            if c.is_digit(10) {
                char.push(c);
            } else if !char.is_empty() {
                let num = char.parse::<i32>().unwrap();
                nums.push(num);
                char.clear();
            }
        }
        let num = char.parse::<i32>().unwrap();
        nums.push(num);
        machines.push(nums);
    }
    let mut ans = 0;
    for one in machines {
        let (a, b) = one_machine_to_win(one).1;
        ans += a * 3 + b
    }
    ans
}

// Button A: X+94, Y+34 (x:80)
// Button B: X+22, Y+67 (y:40)
// Prize: X=8400, Y=5400
// 80*94 + 40*22 = 8400
// 80*34 + 40*67 = 5400

fn one_machine_to_win(nums: Vec<i32>) -> (bool, (i32, i32)) {
    let button_a_x = nums[0];
    let button_a_y = nums[1];
    let button_b_x = nums[2];
    let button_b_y = nums[3];
    let prize_x = nums[4];
    let prize_y = nums[5];

    for y in 0..99 {
        for x in 0..99 {
            if button_a_x * x + button_b_x * y == prize_x
                && button_a_y * x + button_b_y * y == prize_y
            {
                return (true, (x, y));
            }
        }
    }
    (false, (0, 0))
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 480);
}
