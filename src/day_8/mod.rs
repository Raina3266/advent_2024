use std::collections::{HashMap, HashSet};

#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

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

    fn get(&self, location: (isize, isize)) -> Option<char> {
        if location.0 < self.width && location.1 < self.height && location.0 >= 0 && location.1 >= 0
        {
            Some(self.data[location.1 as usize][location.0 as usize])
        } else {
            None
        }
    }
}

pub fn part_1(string: &str) -> i32 {
    let grid = Grid::new(string);
    let mut ans: HashSet<(isize, isize), _> = HashSet::new();

    let mut antennas_map: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let antennas = get_antennas(&grid);
    for antenna in &antennas {
        antennas_map
            .entry(grid.get(*antenna).unwrap())
            .and_modify(|v| v.push(*antenna))
            .or_insert(vec![*antenna]);
    }
    for antenna in antennas_map {
        let pairs = pair_two_antennas(&antenna.1);
        for pair in pairs {
            let first_antenna = antenna.1[pair.0];
            let second_antenna = antenna.1[pair.1];
            let antinodes = put_antinodes(first_antenna, second_antenna, &grid);
            for antinode in antinodes {
                if antennas.clone().iter().any(|a| *a != antinode) {
                    ans.insert(antinode);
                }
            }
        }
    }
    ans.len().try_into().unwrap()
}

fn get_antennas(grid: &Grid) -> Vec<(isize, isize)> {
    let mut result = vec![];
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.data[y as usize][x as usize] != '.' {
                result.push((x, y));
            }
        }
    }
    result
}

fn put_antinodes(
    first_antenna: (isize, isize),
    second_antenna: (isize, isize),
    grid: &Grid,
) -> Vec<(isize, isize)> {
    let mut result: Vec<(isize, isize)> = vec![];
    let mut first_antinode: (isize, isize) = (0, 0);
    let mut second_antinode: (isize, isize) = (0, 0);
    let width_gap = (first_antenna.0 - second_antenna.0).abs();
    let height_gap = (first_antenna.1 - second_antenna.1).abs();
    if first_antenna.0 >= second_antenna.0 && first_antenna.1 >= second_antenna.1 {
        first_antinode.0 = first_antenna.0 + width_gap;
        first_antinode.1 = first_antenna.1 + height_gap;

        second_antinode.0 = second_antenna.0 - width_gap;
        second_antinode.1 = second_antenna.1 - height_gap;
    } else if first_antenna.0 < second_antenna.0 && first_antenna.1 >= second_antenna.1 {
        first_antinode.0 = first_antenna.0 - width_gap;
        first_antinode.1 = first_antenna.1 + height_gap;

        second_antinode.0 = second_antenna.0 + width_gap;
        second_antinode.1 = second_antenna.1 - height_gap;
    } else if first_antenna.0 >= second_antenna.0 && first_antenna.1 < second_antenna.1 {
        first_antinode.0 = first_antenna.0 + width_gap;
        first_antinode.1 = first_antenna.1 - height_gap;

        second_antinode.0 = second_antenna.0 - width_gap;
        second_antinode.1 = second_antenna.1 + height_gap;
    } else if first_antenna.0 < second_antenna.0 && first_antenna.1 < second_antenna.1 {
        first_antinode.0 = first_antenna.0 - width_gap;
        first_antinode.1 = first_antenna.1 - height_gap;

        second_antinode.0 = second_antenna.0 + width_gap;
        second_antinode.1 = second_antenna.1 + height_gap;
    }

    if first_antinode.0 < grid.width
        && first_antinode.1 < grid.height
        && first_antinode.0 >= 0
        && first_antinode.1 >= 0
    {
        result.push(first_antinode);
    }
    if second_antinode.0 < grid.width
        && second_antinode.1 < grid.height
        && second_antinode.0 >= 0
        && second_antinode.1 >= 0
    {
        result.push(second_antinode);
    }
    result
}

fn pair_two_antennas(antenna: &[(isize, isize)]) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = vec![];
    for i in 0..(antenna.len()-1) {
        for j in (i+1)..antenna.len() {
            result.push((i, j))
        }
    }
    result
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 14);
}
