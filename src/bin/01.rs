use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    fn parse(input: &str) -> Vec<(isize, isize)> {
        let mut left = vec![];
        let mut right = vec![];
        for line in input.split('\n') {
            if line.trim().is_empty() {
                continue;
            }
            let mut parts = line.split(" ").filter(|i| i.trim().len() > 0);
            left.push(parts.next().unwrap().trim().parse::<isize>().unwrap());
            right.push(parts.next().unwrap().trim().parse::<isize>().unwrap());
        }

        left.sort();
        right.sort();

        left.into_iter().zip(right.into_iter()).collect()
    }

    Some(
        parse(input)
            .into_iter()
            .map(|(a, b)| (a - b).abs())
            .fold(0isize, |a, c| (a + c)) as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    fn parse(input: &str) -> (Vec<isize>, HashMap<isize, u32>) {
        let mut left = vec![];
        let mut right = vec![];
        for line in input.split('\n') {
            if line.trim().is_empty() {
                continue;
            }
            let mut parts = line.split(" ").filter(|i| i.trim().len() > 0);
            left.push(parts.next().unwrap().trim().parse::<isize>().unwrap());

            right.push(parts.next().unwrap().trim().parse::<isize>().unwrap());
        }

        let mut rcount = HashMap::<isize, u32>::new();
        for r in right.iter() {
            *rcount.entry(*r).or_default() += 1;
        }

        return (left, rcount);
    }

    let (left, rcount) = parse(input);
    return Some(
        left.into_iter()
            .map(|l| l * (*rcount.get(&l).unwrap_or(&0)) as isize)
            .fold(0isize, |a, c| (a + c)) as u32,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
