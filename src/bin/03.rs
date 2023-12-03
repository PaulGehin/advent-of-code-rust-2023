use std::cmp::{max, min};
use std::collections::HashMap;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    for (i, line) in input.lines().enumerate() {
        let mut n: u32 = 0;
        let mut counts = false;
        for (j, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                n = n * 10 + char.to_digit(10).unwrap();
                for ii in max(i as i32 - 1, 0) as usize..min(i + 2, input.lines().count()) {
                    for jj in max(j as i32 - 1, 0) as usize..min(j + 2, line.chars().count()) {
                        let cc = input
                            .lines()
                            .nth(ii)
                            .expect("Line should exist")
                            .chars()
                            .nth(jj)
                            .expect("Character should exist");
                        if (cc != '.') & (!cc.is_ascii_digit()) {
                            counts = true;
                        }
                    }
                }
            } else {
                if counts {
                    counts = false;
                    result += n
                }
                n = 0
            }
        }
        if counts {
            result += n
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let mut n: u32 = 0;
        let mut gear = None;
        for (j, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                n = n * 10 + char.to_digit(10).unwrap();
                for ii in max(i as i32 - 1, 0) as usize..min(i + 2, input.lines().count()) {
                    for jj in max(j as i32 - 1, 0) as usize..min(j + 2, line.chars().count()) {
                        let cc = input
                            .lines()
                            .nth(ii)
                            .expect("Line should exist")
                            .chars()
                            .nth(jj)
                            .expect("Character should exist");
                        if (cc != '.') & (!cc.is_ascii_digit()) & (cc == '*') {
                            gear = Some((ii, jj))
                        }
                    }
                }
            } else {
                if gear.is_some() {
                    match gears.entry(gear.unwrap()) {
                        std::collections::hash_map::Entry::Vacant(e) => {
                            e.insert(vec![n]);
                        }
                        _ => gears.get_mut(&gear.unwrap()).unwrap().push(n),
                    }
                    gear = None;
                }
                n = 0;
            }
        }
        if gear.is_some() {
            match gears.entry(gear.unwrap()) {
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(vec![n]);
                }
                _ => gears.get_mut(&gear.unwrap()).unwrap().push(n),
            }
        }
    }
    Some(
        gears
            .values()
            .filter(|v| v.len() == 2)
            .map(|v| v.iter().product::<u32>())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
