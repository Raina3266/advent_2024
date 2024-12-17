type Number = i64;

#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

pub fn part_1(string: &str) -> Number {
    let machines = get_machine_number(string);

    let mut ans = 0;
    for one in machines {
        let win = one_machine_to_win(one);
        ans += find_cheapest(win);
    }
    ans
}

fn get_machine_number(string: &str) -> Vec<Vec<Number>> {
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
    machines
}
// Button A: X+94, Y+34 (a:80)
// Button B: X+22, Y+67 (b:40)
// Prize: X=8400, Y=5400
// a*button_a_x + b*button_b_x = 8400
// a* button_a_y + b*button_b_y = 5400

fn one_machine_to_win(nums: Vec<Number>) -> Vec<(Number, Number)> {
    let mut winning: Vec<(Number, Number)> = vec![];
    let button_a_x = nums[0];
    let button_a_y = nums[1];
    let button_b_x = nums[2];
    let button_b_y = nums[3];
    let prize_x = nums[4];
    let prize_y = nums[5];

    let left_gap = (button_b_x * button_a_y - button_b_y * button_a_x).abs();
    let right_gap = (prize_x * button_a_y - prize_y * button_a_x).abs();

    if right_gap % left_gap == 0 {
        let button_b = right_gap / left_gap;
        if (prize_x - button_b * button_b_x) % button_a_x == 0 {
            let button_a = (prize_x - button_b * button_b_x) / button_a_x;
            winning.push((button_a, button_b));
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

pub fn part_2(string: &str) -> Number {
    let machines = get_machine_number(string);
    let mut ans = 0;
    for one in machines {
        let win = one_machine_to_win_part_two(one);
        println!("{win:?}");
        ans += find_cheapest(win);
    }
    ans
}

fn one_machine_to_win_part_two(nums: Vec<Number>) -> Vec<(Number, Number)> {
    let mut winning: Vec<(Number, Number)> = vec![];
    let button_a_x = nums[0];
    let button_a_y = nums[1];
    let button_b_x = nums[2];
    let button_b_y = nums[3];
    let prize_x = nums[4] + 10000000000000;
    let prize_y = nums[5] + 10000000000000;

    // button_a_x * button_a + button_b_x * button_b == prize_x
    // button_a_y * button_a + button_b_y * button_b == prize_y

    let left_gap = (button_b_x * button_a_y - button_b_y * button_a_x).abs();
    let right_gap = (prize_x * button_a_y - prize_y * button_a_x).abs();

    if right_gap % left_gap == 0 {
        let button_b = right_gap / left_gap;
        if (prize_x - button_b * button_b_x) % button_a_x == 0 {
            let button_a = (prize_x - button_b * button_b_x) / button_a_x;
            winning.push((button_a, button_b));
        }
        
    }
    winning
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(TEST_INPUT), 480);
}
