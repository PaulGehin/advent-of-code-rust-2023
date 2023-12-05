advent_of_code::solution!(5);

#[derive(Default)]
struct Mapping {
    from: String,
    to: String,
    ranges: Vec<MappingRange>,
}

struct MappingRange {
    dest_start: u64,
    src_start: u64,
    range: u64,
}

impl MappingRange {
    pub fn map(&self, value: u64) -> u64 {
        if value >= self.src_start && value <= self.src_start + self.range {
            return self.dest_start + (value - self.src_start);
        }
        value
    }
}

fn extract_seeds_and_mappings(input: &str) -> (Vec<u64>, Vec<Mapping>) {
    let mut seeds = Vec::new();
    let mut mappings: Vec<Mapping> = Vec::new();
    for line in input.lines() {
        if line.starts_with("seeds: ") {
            seeds = line.split("seeds: ").collect::<Vec<_>>()[1]
                .split(' ')
                .map(|x| x.parse::<u64>().expect("Should be a number"))
                .collect();
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
            let nums: Vec<u64> = line
                .split(' ')
                .map(|x| x.parse::<u64>().expect("Should be a number"))
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
    (seeds, mappings)
}

pub fn part_one(input: &str) -> Option<u64> {
    let values = extract_seeds_and_mappings(input);
    let mut seeds = values.0;
    let mappings = values.1;

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

fn map_value(mut value: u64, mappings: &Vec<Mapping>) -> u64 {
    for mapping in mappings {
        for range in &mapping.ranges {
            if value >= range.src_start && value < range.src_start + range.range {
                value = value - range.src_start + range.dest_start;
                break;
            }
        }
    }
    value
}

pub fn part_two(input: &str) -> Option<u64> {
    let values = extract_seeds_and_mappings(input);
    let seeds = values.0;
    let mappings = values.1;
    let mut min = (u64::MAX, 0);

    for seed in seeds.iter() {
        min.0 = min.0.min(map_value(*seed, &mappings));
    }

    'location: loop {
        let mut value = min.1;

        for mapping in mappings.iter().rev() {
            for range in &mapping.ranges {
                if value >= range.dest_start && value < range.dest_start + range.range {
                    value = value - range.dest_start + range.src_start;
                    break;
                }
            }
        }

        for pair in seeds.chunks(2) {
            if value >= pair[0] && value < pair[0] + pair[1] {
                break 'location;
            }
        }

        min.1 += 1;
    }

    Some(min.1)
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
