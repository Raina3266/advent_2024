
#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn turn(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(PartialEq, Clone)]
struct Grid {
    data: Vec<Vec<char>>,
    width: isize,
    height: isize,
}

impl Grid {
    fn new(string: &str) -> Self {
        let mut data: Vec<Vec<char>> = vec![];
        for l in string.lines() {
            let line: Vec<char> = l.chars().collect();
            data.push(line);
        }
        let width: isize = data[0].len().try_into().unwrap();
        let height: isize = data.len().try_into().unwrap();
        Grid {
            data,
            width,
            height,
        }
    }

    fn get_location(&self, letter: char) -> Vec<(isize, isize)> {
        let mut result = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                if self.data[y as usize][x as usize] == letter {
                    result.push((x, y));
                }
            }
        }
        result
    }

    fn change_into_obstacle(&mut self, x: isize, y: isize) {
        self.data[y as usize][x as usize] = '#'
    }
    fn change_back(&mut self, x: isize, y: isize) {
        self.data[y as usize][x as usize] = '.'
    }
}

#[allow(clippy::collapsible_if)]
pub fn part_1(string: &str) -> i32 {
    let grid = Grid::new(string);
    let mut ans = vec![];

    let mut direction = Direction::Up;
    let mut position = grid.get_location('^')[0];
    let obstacles = grid.get_location('#');

    while position.0 < grid.width && position.1 < grid.height && position.0 >= 0 && position.1 >= 0
    {
        ans.push(position);
        position = move_forward(direction, position);
        if obstacles
            .iter()
            .any(|(aa, bb)| (aa, bb) == (&position.0, &position.1))
        {
            position = one_step_back(&direction, &position);
            direction = direction.turn();
            ans.pop();
        }
    }
    ans.sort();
    ans.dedup();
    ans.len().try_into().unwrap()
}

fn move_forward(current_facing: Direction, current_location: (isize, isize)) -> (isize, isize) {
    let mut step_forward: (isize, isize) = current_location;
    if current_facing == Direction::Up {
        step_forward = (current_location.0, current_location.1 - 1);
    }
    if current_facing == Direction::Down {
        step_forward = (current_location.0, current_location.1 + 1);
    }
    if current_facing == Direction::Left {
        step_forward = (current_location.0 - 1, current_location.1);
    }
    if current_facing == Direction::Right {
        step_forward = (current_location.0 + 1, current_location.1);
    }
    step_forward
}

fn one_step_back(current_facing: &Direction, current_location: &(isize, isize)) -> (isize, isize) {
    let mut step_back: (isize, isize) = *current_location;
    if current_facing == &Direction::Up {
        step_back = (current_location.0, current_location.1 + 1);
    }
    if current_facing == &Direction::Down {
        step_back = (current_location.0, current_location.1 - 1);
    }
    if current_facing == &Direction::Left {
        step_back = (current_location.0 + 1, current_location.1);
    }
    if current_facing == &Direction::Right {
        step_back = (current_location.0 - 1, current_location.1);
    }
    step_back
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 41);
}

#[allow(clippy::collapsible_if)]
pub fn part_2(string: &str) -> i32 {
    let mut grid = Grid::new(string);
    let mut ans = 0;

    let first_direction = Direction::Up;
    let second_direction = Direction::Up;
    let first_guard = grid.get_location('^')[0];
    let second_guard = grid.get_location('^')[0];

    for x in 0..grid.width {
        for y in 0..grid.height {
            if grid.data[y as usize][x as usize] == '.' {
                grid.change_into_obstacle(x, y);
                if check_loop(
                    &grid,
                    first_guard,
                    second_guard,
                    first_direction,
                    second_direction,
                ) {
                    ans += 1
                }
                grid.change_back(x, y);
            }
        }
    }
    ans
}

fn check_loop(
    grid: &Grid,
    mut first_guard: (isize, isize),
    mut second_guard: (isize, isize),
    mut first_direction: Direction,
    mut second_direction: Direction,
) -> bool {
    let obstacles = grid.get_location('#');

    while first_guard.0 < grid.width
        && first_guard.1 < grid.height
        && first_guard.0 >= 0
        && first_guard.1 >= 0
        && second_guard.0 < grid.width
        && second_guard.1 < grid.height
        && second_guard.0 >= 0
        && second_guard.1 >= 0
    {
        first_guard = move_forward(first_direction, first_guard);
        if obstacles
            .iter()
            .any(|(aa, bb)| (aa, bb) == (&first_guard.0, &first_guard.1))
        {
            first_guard = one_step_back(&first_direction, &first_guard);
            first_direction = first_direction.turn();
        }
        first_guard = move_forward(first_direction, first_guard);
        if obstacles
            .iter()
            .any(|(aa, bb)| (aa, bb) == (&first_guard.0, &first_guard.1))
        {
            first_guard = one_step_back(&first_direction, &first_guard);
            first_direction = first_direction.turn();
        }
        first_guard = move_forward(first_direction, first_guard);
        if obstacles
            .iter()
            .any(|(aa, bb)| (aa, bb) == (&first_guard.0, &first_guard.1))
        {
            first_guard = one_step_back(&first_direction, &first_guard);
            first_direction = first_direction.turn();
        }

        second_guard = move_forward(second_direction, second_guard);
        if obstacles
            .iter()
            .any(|(aa, bb)| (aa, bb) == (&second_guard.0, &second_guard.1))
        {
            second_guard = one_step_back(&second_direction, &second_guard);
            second_direction = second_direction.turn();
        }

        if first_guard == second_guard && first_direction == second_direction {
            return true;
        }
    }
    false
}

#[test]
fn feature() {
    let string = TEST_INPUT;
    let grid = Grid::new(string);
    let first_direction = Direction::Up;
    let second_direction = Direction::Up;
    let first_guard = grid.get_location('^')[0];
    let second_guard = grid.get_location('^')[0];
    let result = check_loop(
        &grid,
        first_guard,
        second_guard,
        first_direction,
        second_direction,
    );
    assert_eq!(result, false);
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(TEST_INPUT), 6);
}
