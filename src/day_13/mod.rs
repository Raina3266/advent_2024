type Number = u64;

#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

pub fn part_1(string: &str) -> Number {
    let raw_machines: Vec<&str> = string.split("\n\n").collect();
    let mut machines: Vec<Vec<Number>> = vec![];

    for machine in raw_machines {
        let mut nums = vec![];
        let mut char = String::new();
        for c in machine.chars() {
            if c.is_digit(10) {
                char.push(c);
            } else if !c.is_digit(10) && !char.is_empty() {
                let num = char.parse::<Number>().unwrap();
                nums.push(num);
                char.clear();
            }
        }
        let num = char.parse::<Number>().unwrap();
        nums.push(num);
        machines.push(nums);
    }

    let mut ans = 0;
    for one in machines {
        let win = one_machine_to_win(one);
        ans += find_cheapest(win);
    }
    ans
}

// Button A: X+94, Y+34 (x:80)
// Button B: X+22, Y+67 (y:40)
// Prize: X=8400, Y=5400
// 80*94 + 40*22 = 8400
// 80*34 + 40*67 = 5400

fn one_machine_to_win(nums: Vec<Number>) -> Vec<(Number, Number)> {
    let mut winning: Vec<(Number, Number)> = vec![];
    let button_a_x = nums[0];
    let button_a_y = nums[1];
    let button_b_x = nums[2];
    let button_b_y = nums[3];
    let prize_x = nums[4];
    let prize_y = nums[5];

    for x in 0..1000 {
        for y in 0..1000 {
            if button_a_x * x + button_b_x * y == prize_x
                && button_a_y * x + button_b_y * y == prize_y
            {
                winning.push((x, y));
            }
        }
    }
    winning
}

fn find_cheapest(win: Vec<(Number, Number)>) -> Number {
    if win.is_empty() {
        return 0;
    }
    let mut cheapest = Number::MAX;
    for w in win {
        let token = w.0 * 3 + w.1;
        cheapest = std::cmp::min(cheapest, token);
    }
    cheapest
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 480);
}
