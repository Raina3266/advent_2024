use std::io::{stdin, Read};

type Number = i32;

#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

#[derive(PartialEq, Clone, Debug)]
struct Grid {
    data: Vec<Vec<i32>>,
    width: i32,
    height: i32,
}

impl Grid {
    fn new(point: i32, width: i32, height: i32) -> Self {
        let lines: Vec<i32> = vec![point; width as usize];
        let data: Vec<Vec<i32>> = vec![lines; height as usize];
        Grid {
            data,
            width,
            height,
        }
    }
    fn change_data(&mut self, location_x: i32, location_y: i32) {
        self.data[location_y as usize][location_x as usize] += 1
    }
    fn change_back_data(&mut self, location_x: i32, location_y: i32) {
        self.data[location_y as usize][location_x as usize] = 0
    }
    fn print_cam(&self) {
        println!("=================================");
        println!();
        for row in &self.data {
            for cell in row {
                print!("{cell}");
            }
            println!();
        }
        println!();
    }

    fn print(&self) {}
}

#[derive(PartialEq, Clone, Debug)]
struct Robot {
    location_x: i32,
    location_y: i32,
    move_right: i32,
    move_down: i32,
}

impl Robot {
    fn new(input: &str) -> Self {
        let nums: Vec<i32> = input
            .split(&['=', ',', ' '])
            .filter(|i| i.parse::<i32>().is_ok())
            .map(|i| i.parse().unwrap())
            .collect();
        Robot {
            location_x: nums[0],
            location_y: nums[1],
            move_right: nums[2],
            move_down: nums[3],
        }
    }
}

pub fn part_1(string: &str) -> Number {
    let mut grid = Grid::new(0, 101, 103);
    for line in string.lines() {
        let mut robot = Robot::new(line);
        for _ in 0..100 {
            move_one_step(&mut robot, &grid)
        }
        grid.change_data(robot.location_x, robot.location_y);
        println!("{grid:?}");
    }
    calculation(grid)
}

#[allow(clippy::collapsible_if, clippy::collapsible_else_if)]
fn move_one_step(robot: &mut Robot, grid: &Grid) {
    if robot.move_right > 0 {
        robot.location_x = (robot.location_x + robot.move_right) % grid.width;
    } else {
        robot.location_x = (grid.width + robot.location_x + robot.move_right) % grid.width;
    }
    if robot.move_down > 0 {
        robot.location_y = (robot.location_y + robot.move_down) % grid.height;
    } else {
        robot.location_y = (grid.height + robot.location_y + robot.move_down) % grid.height;
    }
}

fn calculation(grid: Grid) -> i32 {
    let mid_width: usize = ((grid.width - 1) / 2).try_into().unwrap();
    let mid_height: usize = ((grid.height - 1) / 2).try_into().unwrap();
    let one: Vec<i32> = grid.data[0..mid_height]
        .iter()
        .flat_map(|row| row[0..mid_width].to_vec())
        .collect();
    let two: Vec<i32> = grid.data[0..mid_height]
        .iter()
        .flat_map(|row| row[mid_width + 1..].to_vec())
        .collect();
    let three: Vec<i32> = grid.data[mid_height + 1..]
        .iter()
        .flat_map(|row| row[0..mid_width].to_vec())
        .collect();
    let four: Vec<i32> = grid.data[mid_height + 1..]
        .iter()
        .flat_map(|row| row[mid_width + 1..].to_vec())
        .collect();
    one.iter().sum::<i32>()
        * two.iter().sum::<i32>()
        * three.iter().sum::<i32>()
        * four.iter().sum::<i32>()
}

#[test]
fn test_one_step() {
    let input = "p=2,4 v=2,-3";
    let mut robot = Robot::new(input);
    dbg!(&robot);
    let grid = Grid::new(0, 11, 7);

    move_one_step(&mut robot, &grid);
    move_one_step(&mut robot, &grid);
    move_one_step(&mut robot, &grid);
    move_one_step(&mut robot, &grid);
    move_one_step(&mut robot, &grid);
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 12);
}

const SKIP: usize = 8256;
pub fn part_2(string: &str) {
    let mut grid = Grid::new(0, 101, 103);
    let mut robots: Vec<_> = string.lines().map(Robot::new).collect();

    let mut time = SKIP;
    for _ in 0..SKIP {
        for robot in &mut robots {
            move_one_step(robot, &grid);
        }
    }

    loop {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        drop(line);

        time += 1;

        for robot in &mut robots {
            move_one_step(robot, &grid);
        }

        print_robots(&robots, grid.width, grid.height);
        println!();
        println!("time = {time}");
    }
}

fn print_robots(robots: &[Robot], width: i32, height: i32) {
    for x in 0..width {
        for y in 0..height {
            let num_robots = robots
                .iter()
                .filter(|robot| robot.location_x == x && robot.location_y == y)
                .count();
            let char = match num_robots {
                0 => ".",
                _ => "#",
            };

            print!("{char}");
        }
        println!();
    }
}
