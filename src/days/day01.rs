use itertools::Itertools;

fn part1(input: Vec<u32>) -> u32 {
    input.into_iter().combinations(2)
        .find(|c| c.into_iter().copied().sum::<u32>() == 2020).expect("No solution")
        .iter().copied().product()
}

fn part2(input: Vec<u32>) -> u32 {
    input.into_iter().combinations(3)
        .find(|c| c.into_iter().copied().sum::<u32>() == 2020).expect("No solution")
        .iter().copied().product()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use days::day01::{part1, part2};

    fn get_input() -> Vec<u32> {
        let input = fs::read_to_string("input/01.txt").expect("Unable to open file");

        input.trim()
            .split('\n')
            .map(|s| s.trim()).map(|s| s.parse().unwrap())
            .collect()
    }

    #[test]
    fn test_part_1() {
        let result = part1(get_input());
        println!("Part 1: {}", result)
    }

    #[test]
    fn test_part_2() {
        let result = part2(get_input());
        println!("Part 2: {}", result)
    }
}