use itertools::Itertools;
use std::collections::BinaryHeap;
use std::ops::Deref;

pub struct Calories(BinaryHeap<u32>);

impl Deref for Calories {
    type Target = BinaryHeap<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromIterator<u32> for Calories {
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Self {
        Self(BinaryHeap::from(Vec::from_iter(iter)))
    }
}

#[aoc_generator(day1, part1, heap)]
#[aoc_generator(day1, part2, heap)]
pub fn parse_input_heap(input: &str) -> Calories {
    input
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .collect()
}

#[aoc(day1, part1, heap)]
pub fn solve_part1_heap(input: &Calories) -> u32 {
    *input.iter().nth(0).unwrap()
}

#[aoc(day1, part2, heap)]
pub fn solve_part2_heap(input: &Calories) -> u32 {
    input.iter().take(3).sum::<u32>()
}

pub struct Elf(Vec<u32>);

impl Deref for Elf {
    type Target = Vec<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromIterator<u32> for Elf {
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

#[aoc_generator(day1, part1, sorting)]
#[aoc_generator(day1, part2, sorting)]
pub fn parse_input_sorting(input: &str) -> Vec<Elf> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.trim().parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day1, part1, sorting)]
pub fn solve_part1_sorting(input: &Vec<Elf>) -> u32 {
    input
        .iter()
        .map(|calories| calories.iter().sum::<u32>())
        .sorted_by(|a, b| b.cmp(&a))
        .nth(0)
        .unwrap_or_default()
}

#[aoc(day1, part2, sorting)]
pub fn solve_part2_sorting(input: &Vec<Elf>) -> u32 {
    input
        .iter()
        .map(|calories| calories.iter().sum::<u32>())
        .sorted_by(|a, b| b.cmp(&a))
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            solve_part1_sorting(&parse_input_sorting(
                "1000\n\
                2000\n\
                3000\n\
                \n\
                4000\n\
                \n\
                5000\n\
                6000\n\
                \n\
                7000\n\
                8000\n\
                9000\n\
                \n\
                10000"
            )),
            24000
        );

        assert_eq!(
            solve_part1_heap(&parse_input_heap(
                "1000\n\
                2000\n\
                3000\n\
                \n\
                4000\n\
                \n\
                5000\n\
                6000\n\
                \n\
                7000\n\
                8000\n\
                9000\n\
                \n\
                10000"
            )),
            24000
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2_sorting(&parse_input_sorting(
                "1000\n\
                2000\n\
                3000\n\
                \n\
                4000\n\
                \n\
                5000\n\
                6000\n\
                \n\
                7000\n\
                8000\n\
                9000\n\
                \n\
                10000"
            )),
            45000
        );

        assert_eq!(
            solve_part2_heap(&parse_input_heap(
                "1000\n\
                2000\n\
                3000\n\
                \n\
                4000\n\
                \n\
                5000\n\
                6000\n\
                \n\
                7000\n\
                8000\n\
                9000\n\
                \n\
                10000"
            )),
            45000
        );
    }
}
