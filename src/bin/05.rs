advent_of_code::solution!(5);

#[derive(Default)]
struct Mapping {
    from: String,
    to: String,
    ranges: Vec<MappingRange>,
}

struct MappingRange {
    dest_start: u32,
    src_start: u32,
    range: u32,
}

impl MappingRange {
    pub fn map(&self, value: u32) -> u32 {
        if value >= self.src_start && value <= self.src_start + self.range {
            return self.dest_start + (value - self.src_start);
        }
        value
    }
}

fn compute(input: &str, mut seeds: Vec<u32>) -> Option<u32> {
    let mut mappings: Vec<Mapping> = Vec::new();
    for line in input.lines() {
        if line.starts_with("seeds: ") {
            continue;
        } else if line.ends_with(" map:") {
            let categories: Vec<&str> = line.split(" map:").collect::<Vec<_>>()[0]
                .split("-to-")
                .collect();
            mappings.push(Mapping {
                from: categories[0].to_string(),
                to: categories[1].to_string(),
                ranges: Vec::new(),
            });
        } else if !line.is_empty() {
            let nums: Vec<u32> = line
                .split(' ')
                .map(|x| x.parse::<u32>().expect("Should be a number"))
                .collect();
            mappings
                .last_mut()
                .expect("Should have at least one mapping")
                .ranges
                .push(MappingRange {
                    dest_start: nums[0],
                    src_start: nums[1],
                    range: nums[2],
                });
        }
    }

    let mut current_category = "seed".to_string();
    while current_category != "location" {
        for mapping in mappings.iter() {
            if mapping.from == current_category {
                current_category = mapping.to.clone();
                for seed in seeds.iter_mut() {
                    for range in mapping.ranges.iter() {
                        let old_seed = *seed;
                        *seed = range.map(*seed);
                        if old_seed != *seed {
                            break;
                        }
                    }
                }
                break;
            }
        }
    }

    seeds.iter().min().copied()
}

fn extract_seeds(line: &str) -> Vec<u32> {
    line.split("seeds: ").collect::<Vec<_>>()[1]
        .split(' ')
        .map(|x| x.parse::<u32>().expect("Should be a number"))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    for line in input.lines() {
        if line.starts_with("seeds: ") {
            return compute(input, extract_seeds(line));
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    for line in input.lines() {
        if line.starts_with("seeds: ") {
            let mut seeds: Vec<u32> = Vec::new();
            let seed_parts: Vec<u32> = extract_seeds(line);
            for i in (0..seed_parts.len()).step_by(2) {
                let seed_start = seed_parts[i];
                let seed_len = seed_parts[i + 1];
                for j in 0..seed_len {
                    seeds.push(seed_start + j);
                }
            }
            return compute(input, seeds);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
