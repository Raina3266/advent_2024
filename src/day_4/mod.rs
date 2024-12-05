#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

struct Grid<'a> {
    data: &'a str,
    width: isize,
    height: isize,
}

impl<'a> Grid<'a> {
    fn new(string: &'a str) -> Self {
        let w: isize = string
            .lines()
            .next()
            .unwrap()
            .chars()
            .count()
            .try_into()
            .unwrap();
        let h: isize = string.lines().count().try_into().unwrap();
        Grid {
            data: string,
            width: w,
            height: h,
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<char> {
        if x < self.width && y < self.height && x >= 0 && y >= 0 {
            return self
                .data
                .lines()
                .nth(y as usize)
                .unwrap()
                .chars()
                .nth(x as usize);
        } else {
            None
        }
    }

    fn check(&self, x: isize, y: isize, letter: char) -> bool {
        self.get(x, y) == Some(letter)
    }
}
#[allow(clippy::collapsible_if)]
pub fn part_1(string: &str) -> i32 {
    let grid = Grid::new(string);
    let mut ans = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.check(x, y, 'X') {
                let left = x - 1;
                let leftleft = x - 2;
                let leftleftleft = x - 3;
                if grid.check(left, y, 'M')
                    && grid.check(leftleft, y, 'A')
                    && grid.check(leftleftleft, y, 'S')
                {
                    ans += 1;
                }
                let right = x + 1;
                let rightright = x + 2;
                let rightrightright = x + 3;
                if grid.check(right, y, 'M')
                    && grid.check(rightright, y, 'A')
                    && grid.check(rightrightright, y, 'S')
                {
                    ans += 1;
                }
                let up = y + 1;
                let upup = y + 2;
                let upupup = y + 3;
                if grid.check(x, up, 'M') && grid.check(x, upup, 'A') && grid.check(x, upupup, 'S')
                {
                    ans += 1;
                }
                let down = y - 1;
                let downdown = y - 2;
                let downdowndown = y - 3;
                if grid.check(x, down, 'M')
                    && grid.check(x, downdown, 'A')
                    && grid.check(x, downdowndown, 'S')
                {
                    ans += 1;
                }
                if grid.check(left, down, 'M')
                    && grid.check(leftleft, downdown, 'A')
                    && grid.check(leftleftleft, downdowndown, 'S')
                {
                    ans += 1;
                }
                if grid.check(left, up, 'M')
                    && grid.check(leftleft, upup, 'A')
                    && grid.check(leftleftleft, upupup, 'S')
                {
                    ans += 1;
                }
                if grid.check(right, down, 'M')
                    && grid.check(rightright, downdown, 'A')
                    && grid.check(rightrightright, downdowndown, 'S')
                {
                    ans += 1;
                }
                if grid.check(right, up, 'M')
                    && grid.check(rightright, upup, 'A')
                    && grid.check(rightrightright, upupup, 'S')
                {
                    ans += 1;
                }
            }
        }
    }
    ans
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 18);
}

pub fn part_2(string: &str) -> i32 {
    let grid = Grid::new(string);
    let mut ans = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.check(x, y, 'A') {
                let left = x - 1;
                let right = x + 1;
                let up = y - 1;
                let down = y + 1;
                if (grid.check(left, up, 'M') && grid.check(right, down, 'S')
                    && grid.check(right, up, 'M') && grid.check(left, down, 'S'))
                    || (grid.check(left, up, 'M') && grid.check(right, down, 'S')
                    && grid.check(left, down, 'M') && grid.check(right, up, 'S'))
                    || (grid.check(right, down, 'M') && grid.check(left, up, 'S')
                    && grid.check(left, down, 'M') && grid.check(right, up, 'S'))
                    || (grid.check(right, down, 'M') && grid.check(left, up, 'S')
                    && grid.check(right, up, 'M') && grid.check(left, down, 'S'))
                {
                    ans += 1
                }
            }
        }
    }
    ans
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(TEST_INPUT), 9);
}
