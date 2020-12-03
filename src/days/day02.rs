use regex::Regex;

#[derive(Debug, Clone)]
struct Password {
    num1: u8,
    num2: u8,
    letter: char,
    password: String,
}


impl Password {
    fn from_str(s: String) -> Password {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
         }
        let captures = RE.captures(s.as_str()).expect("Did not match regex");

        let p = Password {
            num1: captures.get(1).unwrap().as_str().parse().expect("invalid min"),
            num2: captures.get(2).unwrap().as_str().parse().expect("invalid max"),
            letter: captures.get(3).unwrap().as_str().parse().expect("invalid char"),
            password: String::from(captures.get(4).unwrap().as_str()),
        };

        p
    }

    fn is_valid(self) -> bool {
        let c = self.password.chars().into_iter().filter(|c| *c == self.letter).count();
        c >= self.num1 as usize && c <= self.num2 as usize
    }

    fn is_valid2(self) -> bool {
        let null = '\0';
        let chars: Vec<char> = self.password.chars().collect();

        let c1 = chars.get(self.num1 as usize - 1).unwrap_or(&null);
        let c2 = chars.get(self.num2 as usize - 1).unwrap_or(&null);

        (*c1 == self.letter) ^ (*c2 == self.letter)
    }
}

fn part1(passwords: Vec<Password>) -> u32 {
    passwords.into_iter()
        .map(|s| s.is_valid())
        .filter(|s| *s)
        .count() as u32
}

fn part2(passwords: Vec<Password>) -> u32 {
    passwords.into_iter()
        .map(|s| s.is_valid2())
        .filter(|s| *s)
        .count() as u32
}

#[cfg(test)]
mod tests {
    use std::fs;
    use days::day02::{Password, part1, part2};

    fn get_input() -> Vec<Password> {
        let input = fs::read_to_string("input/02.txt").expect("Unable to open file");

        input.trim().split('\n')
            .map(|s| s.trim())
            .map(|s| Password::from_str(s.into()))
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