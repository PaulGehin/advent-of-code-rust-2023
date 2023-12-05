advent_of_code::solution!(4);

pub fn point(line: &str) -> usize {
    let mut numbers = line
        .split(':')
        .nth(1)
        .expect("Line must have ID and numbers")
        .split('|')
        .map(|x| {
            x.split(' ')
                .filter(|y| !y.is_empty())
                .map(|y| y.parse::<u32>().unwrap())
        });
    let winnings: Vec<u32> = numbers.next().expect("Must have winning numbers").collect();
    numbers
        .next()
        .expect("Must have actual numbers")
        .filter(|x| winnings.contains(x))
        .count()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(point).map(|n| 1 << n >> 1).sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut resu: Vec<u32> = input.lines().map(|_| 1).collect();

    for (i, n) in input.lines().map(point).enumerate() {
        if n.ne(&0) {
            for j in i + 1..n + i + 1 {
                resu[j] += resu[i]
            }
        }
    }

    Some(resu.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
