struct Map {
    rows: Vec<Vec<Type>>
}

#[derive(Debug, Copy, Clone)]
enum Type {
    TREE,
    EMPTY,
}

impl Type {
    fn from_char(c: char) -> Type {
        match c {
            '#' => Type::TREE,
            '.' => Type::EMPTY,
            _ => panic!("Unrecognized character")
        }
    }
}

impl Map {
    fn val_at(&self, x: u32, y: u32) -> Option<Type> {
        match self.rows.get((y - 1) as usize) {
            Some(row) => {
                let wrapping = row.len();
                match x.checked_rem(wrapping as u32) {
                    Some(mut remainder) => {
                        if remainder == 0 {
                            remainder = wrapping as u32
                        }
                        let z = *row.get((remainder - 1) as usize).unwrap();
                        Some(z)
                    }
                    None => None
                }
            }
            None => None
        }
    }
}

struct Position(u32, u32);

impl Position {
    fn apply_slope(&mut self, slope: &Position) {
        self.0 += slope.0;
        self.1 += slope.1;
    }
}

fn part1(input: Map, slope: Position) -> u32 {
    let mut p = Position(1, 1);
    let mut trees = 0;
    loop {
        match input.val_at(p.0, p.1) {
            Some(t) => {
                match t {
                    Type::TREE => {
                        trees += 1;
                    }
                    _ => ()
                }
            }
            None => {
                return trees;
            }
        };

        p.apply_slope(&slope);
    }
}


#[cfg(test)]
mod tests {
    use std::fs;
    use days::day03::{Type, Map, Position, part1};

    fn get_input() -> Map {
        let input = fs::read_to_string("input/03.txt").expect("Unable to open file");
        let rows = input
            .trim()
            .lines()
            .map(str::trim)
            .map(|s| s.chars().map(Type::from_char).collect())
            .collect();

        Map { rows }
    }

    #[test]
    fn test_part_1() {
        println!("Part 1: {}", part1(get_input(), Position(3, 1)))
    }

    #[test]
    fn test_part_2() {
        let answer: u64 = [(1,1), (3,1), (5,1), (7,1), (1,2)].iter().map(|s| part1(get_input(), Position(s.0, s.1)) as u64).product();
        println!("Part 2: {}", answer)
    }
}