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
        let w: isize = (string.lines().next().unwrap().chars().count() - 1)
            .try_into()
            .unwrap();
        let h: isize = (string.lines().count() - 1).try_into().unwrap();
        Grid {
            data: string,
            width: w,
            height: h,
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<char> {
        let index = x + self.width * y;
        if x < self.width && y < self.height && x >= 0 && y >= 0 {
            return self.data.chars().nth(index.try_into().unwrap());
        } else {
            None
        }
    }
    fn check(&self, x: isize, y: isize, letter: char) -> bool {
        match self.get(x, y) {
            Some(c) => c == letter,
            None => false,
        }
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
                if grid.check(left, y, 'M') {
                    if grid.check(leftleft, y, 'A') {
                        if grid.check(leftleftleft, y, 'S') {
                            ans += 1;
                        }
                    }
                } 
                let right = x + 1;
                let rightright = x + 2;
                let rightrightright = x + 3;
                if grid.check(right, y, 'M') {
                    if grid.check(rightright, y, 'A') {
                        if grid.check(rightrightright, y, 'S') {
                            ans += 1;
                        }
                    }
                } 
                let up = y + 1;
                let upup = y + 2;
                let upupup = y + 3;
                if grid.check(x, up, 'M') {
                    if grid.check(x, upup, 'A') {
                        if grid.check(x, upupup, 'S') {
                            ans += 1;
                        }
                    }
                } 
                let down = y - 1;
                let downdown = y - 2;
                let downdowndown = y - 3;
                if grid.check(x, down, 'M') {
                    if grid.check(x, downdown, 'A') {
                        if grid.check(x, downdowndown, 'S') {
                            ans += 1;
                        }
                    }
                }
                if grid.check(left, down, 'M') {
                    if grid.check(left, downdown, 'A') {
                        if grid.check(left, downdowndown, 'S') {
                            ans += 1;
                        }
                    }
                }
                if grid.check(left, up, 'M') {
                    if grid.check(left, upup, 'A') {
                        if grid.check(left, upupup, 'S') {
                            ans += 1;
                        }
                    }
                }
                if grid.check(right, down, 'M') {
                    if grid.check(rightright, downdown, 'A') {
                        if grid.check(rightrightright, downdowndown, 'S') {
                            ans += 1;
                        }
                    }
                }
                if grid.check(right, up, 'M') {
                    if grid.check(rightright, upup, 'A') {
                        if grid.check(rightrightright, upupup, 'S') {
                            ans += 1;
                        }
                    }
                }
            }
                
        }

    }
    ans
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 1);
}
