advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();
    let problems: Vec<Vec<&str>> = (0..lines.first()?.len())
        .map(|x| lines.iter().map(|line| line[x]).collect())
        .collect();
    let results: Vec<u64> = problems
        .iter()
        .map(|problem| {
            let numbers: Vec<u64> = problem
                .iter()
                .take(problem.len() - 1)
                .map(|num| num.parse().unwrap())
                .collect();
            let operation = match *problem.last().unwrap() {
                "*" => |a, b| -> u64 { a * b },
                "+" => |a, b| -> u64 { a + b },
                _ => panic!(),
            };
            numbers.iter().copied().reduce(operation).unwrap()
        })
        .collect();
    Some(results.iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines();
    let operators: Vec<&u8> = lines
        .clone()
        .last()
        .iter()
        .flat_map(|line| {
            line.split_whitespace()
                .flat_map(|operator| operator.as_bytes())
        })
        .collect();

    let mut numbers: Vec<Vec<char>> = vec![];
    for line in lines.clone().take(lines.count() - 1) {
        for (x, char) in line.chars().enumerate() {
            if numbers.len() <= x {
                numbers.push(vec![char]);
            } else {
                numbers[x].push(char);
            }
        }
    }
    let problems: Vec<&[Vec<char>]> = numbers
        .as_slice()
        .split(|chars| chars.iter().all(|&char| char == ' '))
        .collect();
    let result: u64 = problems
        .iter()
        .enumerate()
        .map(|(i, problem_numbers)| {
            let operator = operators[i];
            problem_numbers
                .iter()
                .map(|digits| digits.iter().collect::<String>().trim().parse::<u64>().unwrap())
                .reduce(|acc, e| match operator {
                    b'+' => acc + e,
                    b'*' => acc * e,
                    _ => panic!(),
                })
                .unwrap()
        })
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
