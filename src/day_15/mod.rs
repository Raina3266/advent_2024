#[cfg(test)]
pub const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

#[derive(PartialEq, Clone, Debug)]
struct Grid {
    data: Vec<Vec<Node>>,
    width: usize,
    height: usize,
    robot: (usize, usize),
}

impl Grid {
    fn new(input: &str) -> Grid {
        let mut data: Vec<Vec<Node>> = Vec::new();
        let mut height = 0;
        let mut robot = (0, 0);
        for line in input.lines().rev() {
            height += 1;
            data.push(
                line.chars()
                    .enumerate()
                    .map(|(width, step)| match step {
                        'O' => Node::Box,
                        '#' => Node::Wall,
                        '.' => Node::Empty,
                        '@' => {
                            robot.0 = width;
                            robot.1 = height;
                            Node::Robot
                        }
                        _ => unreachable!(),
                    })
                    .collect(),
            )
        }
        let width = data[0].len();
        Grid {
            data,
            width,
            height,
            robot,
        }
    }

    fn move_once(&mut self, x: usize, y: usize, direction: &Move) {
        if self.moveable(x, y, direction) {
            self.data[y][x] = Node::Empty;
            self.move_boxes(x, y, direction);
            match direction {
                Move::Up => {
                    self.data[y + 1][x] = Node::Robot;
                    self.robot = (x, y + 1);
                }
                Move::Down => {
                    self.data[y - 1][x] = Node::Robot;
                    self.robot = (x, y - 1);
                }
                Move::Right => {
                    self.data[y][x + 1] = Node::Robot;
                    self.robot = (x + 1, y);
                }
                Move::Left => {
                    self.data[y][x - 1] = Node::Robot;
                    self.robot = (x - 1, y);
                }
            }
        }
    }
    fn move_boxes(&mut self, x: usize, y: usize, direction: &Move) {
        match direction {
            Move::Up => {
                if self.data[y + 1][x] == Node::Box {
                    self.move_boxes(x, y + 1, direction);
                    self.data[y + 2][x] = Node::Box;
                }
            }
            Move::Down => {
                if self.data[y - 1][x] == Node::Box {
                    self.move_boxes(x, y - 1, direction);
                    self.data[y - 2][x] = Node::Box;
                }
            }
            Move::Left => {
                if self.data[y][x - 1] == Node::Box {
                    self.move_boxes(x - 1, y, direction);
                    self.data[y][x - 1] = Node::Box;
                }
            }
            Move::Right => {
                if self.data[y][x + 1] == Node::Box {
                    self.move_boxes(x + 1, y, direction);
                    self.data[y][x + 1] = Node::Box;
                }
            }
        }
    }

    fn moveable(&self, x: usize, y: usize, direction: &Move) -> bool {
        match direction {
            Move::Up => match self.data[y + 1][x] {
                Node::Empty => true,
                Node::Box => self.moveable(x, y + 1, direction),
                Node::Wall | Node::Robot => false,
            },
            Move::Down => match self.data[y - 1][x] {
                Node::Empty => true,
                Node::Box => self.moveable(x, y - 1, direction),
                Node::Wall | Node::Robot => false,
            },
            Move::Right => match self.data[y][x + 1] {
                Node::Empty => true,
                Node::Box => self.moveable(x + 1, y, direction),
                Node::Wall | Node::Robot => false,
            },
            Move::Left => match self.data[y][x - 1] {
                Node::Empty => true,
                Node::Box => self.moveable(x - 1, y, direction),
                Node::Wall | Node::Robot => false,
            },
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Node {
    Empty,
    Box,
    Wall,
    Robot,
}

#[derive(Clone, Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn new(input: &str) -> Vec<Move> {
        input
            .chars()
            .filter_map(|step| {
                match step {
                    '^' => Some(Move::Up),
                    'v' => Some(Move::Down),
                    '>' => Some(Move::Right),
                    '<' => Some(Move::Left),
                    _ => None, // unrecognized chars are silently skipped
                }
            })
            .collect()
    }
}

pub fn part_1(string: &str) {
    let mut iter = string.split("\n\n");
    let mut grid = Grid::new(iter.next().unwrap());
    let moves = Move::new(iter.next().unwrap());

    for one_move in moves {
        grid.move_once(grid.robot.0, grid.robot.1, &one_move);
    }
    println!("{grid:?}");
}

// #[test]
// fn part_1_test() {
//     assert_eq!(part_1(TEST_INPUT), 104);
// }

pub fn part_2(string: &str) -> i32 {
    todo!()
}

// #[test]
// fn part_2_test() {
//     assert_eq!(part_2(TEST_INPUT), 31);
// }
