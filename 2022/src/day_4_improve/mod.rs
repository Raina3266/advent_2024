use std::{ops::RangeInclusive, str::FromStr};

#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

struct Line {
    first: RangeInclusive<i32>,
    second: RangeInclusive<i32>,
}

/// Only checks if inner is 100% inside outer
fn range_entirely_contains(outer: RangeInclusive<i32>, inner: RangeInclusive<i32>) -> bool {
    outer.start() <= inner.start() && outer.end() >= inner.end()
}

// std::ops::Range
// - implements Iterator
// - there is a general rule that types should not implement Iterator and Copy
//   - easy to make mistakes if this is true
// - does not implement Copy
// std::range::Range
// - does implement Copy
// - does not implement Iterator
//   - but does implement IntoIterator
impl Line {
    fn parse_elf(elf: &str) -> Option<RangeInclusive<i32>> {
        let (start, end) = elf.split_once("-")?;
        let start = start.parse().ok()?;
        let end = end.parse().ok()?;

        Some(start..=end)
    }

    fn parse_line(line: &str) -> Option<Self> {
        let (first, second) = line.split_once(",")?;
        let first = Self::parse_elf(first)?;
        let second = Self::parse_elf(second)?;

        Some(Self { first, second })
    }

    pub fn one_entirely_contains_other(&self) -> bool {
        range_entirely_contains(self.first.clone(), self.second.clone())
            || range_entirely_contains(self.second.clone(), self.first.clone())
    }
}

// `.into_iter()` comes from the trait IntoIterator
// `.iter()` and `.iter_mut()` are conventions (i.e. many types implement a
// function with this name but it has no special meaning). For example,
// `Vec::iter` takes a &self and returns a `std::slice::Iter<'a, T>`
//
// The convention is (for types that are collections, for example `MyList<T>`):
// - you should `impl IntoIterator for MyList<T>`. It should create an iterator that yields `T`s
// - you should `impl IntoIterator for &'a MyList<T>`. It should create an iterator that yields `&'a T`s
// - you should `impl IntoIterator for &'a mut MyList<T>`. It should create an iterator that yields `&'a mut T`s (this one is more loose, for example, &mut HashMap<K, V> iterates over (&K, &mut V))
// - you should create a non-trait method called `.iter()`. It should do the same thing as `(&my_list).into_iter()`
// - you should create a non-trait method called `.iter_mut()`. It should do the same thing as `(&mut my_list).into_iter()`
//

// fn foo() {
//     // enumerate could be .map, .filter, etc.
//     //
//     let iter = whatever();
//     for x in iter.enumerate() {}  // works

//     let into_iter = whatever();
//     for x in into_iter.enumerate() {}  // ERROR

//     let into_iter = whatever();
//     for x in into_iter.into_iter().enumerate() {}  // works

//     // but what if you want to iterate over references

//     let into_iter = whatever();
//     for x in (&into_iter).into_iter().enumerate() {}  // works, but ugly

//     let into_iter = whatever();
//     for x in into_iter.iter().enumerate() {}  // works

//     // standard rust function naming
//     // T: into_foo
//     // &T: foo
//     // &mut T: foo_mut
// }

struct ElfIds {
    start: i32,
    end: i32,
}

impl ElfIds {
    fn new_group(input: &str) -> (Self, Self) {
        let mut group = input.split(',').map(|elf| {
            let mut id = elf.split('-').map(|id| id.parse::<i32>().unwrap());
            (id.next().unwrap(), id.next().unwrap())
        });
        let first_elf = group.next().unwrap();
        let second_elf = group.next().unwrap();
        (
            Self {
                start: first_elf.0,
                end: first_elf.1,
            },
            Self {
                start: second_elf.0,
                end: second_elf.1,
            },
        )
    }
    fn is_contain_mutual(&self, other: ElfIds) -> bool {
        self.start <= other.start && self.end >= other.end
            || other.start <= self.start && other.end >= self.end
    }
    fn is_overlap(&self, other: ElfIds) -> bool {
        self.start < other.end && self.end >= other.start
            || other.start < self.end && self.start <= other.end
            || self.start == other.start
            || self.end == other.end
    }
}

pub fn part_1(string: &str) -> i32 {
    // filter_map:
    // - self: Iterator<T>
    // - f   : T -> Option<S>
    // 
    // returns: Iterator<S>
    string
        .lines()
        .filter_map(|line| Line::parse_line(line))
        .filter(|line| line.one_entirely_contains_other())
        .count() as i32
    // let mut count = 0;
    // for pair in string.lines() {
    //     let (elf1, elf2) = ElfIds::new_group(pair);

    //     if elf1.is_contain_mutual(elf2) {
    //         count += 1;
    //     }
    // }
    // count
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(INPUT), 2);
}

pub fn part_2(string: &str) -> i32 {
    let mut count = 0;
    for pair in string.lines() {
        let (elf1, elf2) = ElfIds::new_group(pair);

        if elf1.is_overlap(elf2) {
            count += 1;
        }
    }
    count
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(INPUT), 825);
}
