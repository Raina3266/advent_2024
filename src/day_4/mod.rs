#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

struct Grid {
    data: Vec<Vec<char>>, // 2D grid of characters
    width: usize,
    height: usize,
}

impl Grid {
    fn new(string: &str) -> Self {
        let data: Vec<Vec<char>> = string.lines().map(|line| line.chars().collect()).collect();
        let height = data.len();
        let width = data[0].len();
        Grid { data, width, height }
    }

    fn get(&self, x: isize, y: isize) -> Option<char> {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            Some(self.data[y as usize][x as usize])
        } else {
            None
        }
    }

    fn count_xmas(&self) -> usize {
        let directions = [
            (1, 0),   // Right
            (-1, 0),  // Left
            (0, 1),   // Down
            (0, -1),  // Up
            (1, 1),   // Diagonal down-right
            (-1, -1), // Diagonal up-left
            (1, -1),  // Diagonal down-left
            (-1, 1),  // Diagonal up-right
        ];

        let mut count = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                for (dx, dy) in directions {
                    if self.check_word(x as isize, y as isize, dx, dy, "XMAS") {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    fn check_word(&self, x: isize, y: isize, dx: isize, dy: isize, word: &str) -> bool {
        word.chars()
            .enumerate()
            .all(|(i, c)| self.get(x + i as isize * dx, y + i as isize * dy) == Some(c))
    }
}

#[allow(clippy::collapsible_if)]
pub fn part_1(string: &str) -> i32 {
    let grid = Grid::new(string);
    let count = grid.count_xmas();
    count.try_into().unwrap()
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 18);
}
