use regex::bytes::Regex;
use std::ptr::swap;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    //parse
    //for each machine: use a bitmap
    // buttons: vec of indexes
    let mut total_presses = 0;

    let re = Regex::new(r"\[(?<machine>.+)\] (?<buttons>.+) \{.+\}").unwrap();
    'machine: for line in input.lines() {
        let caps = re.captures(line.as_bytes())?;
        let machine: Vec<bool> = caps["machine"].iter().map(|&b| b == b'#').collect();

        let buttons: Vec<Vec<usize>> = caps["buttons"]
            .split(|b| *b == b' ')
            .map(|button_str| {
                button_str[1..button_str.len() - 1]
                    .split(|b| *b == b',')
                    .map(|digits| (digits[0] - b'0') as usize)
                    .collect()
            })
            .collect();

        let mut prev_presses: Vec<Vec<usize>> = vec![vec![]; buttons.len()];

        for n in 1..=buttons.len() {
            let mut next_presses = vec![];
            for prev in &prev_presses {
                let &last = prev.last().unwrap_or(&0);
                for x in last..buttons.len() {
                    let next = [&prev[..], &[x]].concat();
                    let mut result = vec![false; machine.len()];
                    for &press in &next {
                        for &flip in &buttons[press]{
                            result[flip] = !result[flip];
                        }
                    }
                    if result == machine {
                        total_presses += n;
                        continue 'machine;
                    }
                    next_presses.push(next);
                }
            }

            std::mem::swap(&mut prev_presses, &mut next_presses);
        }
    }

    Some(total_presses as u64)

    //for each machine:
    //  presses: vec of indexes of buttons pressed
    //  n : buttons pressed
    //  loop
    //      clear new presses
    //      press a button (positioned after prev in solutions) and push indexes to new solutions
    //          break if correct state
    //      swap new solutions and solutions
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total_presses = 0;

    let re = Regex::new(r"\[.+\] (?<buttons>.+) \{(?<joltage>.+)\}").unwrap();
    'machine: for line in input.lines() {
        println!("machine");
        let caps = re.captures(line.as_bytes())?;
        let machine: Vec<u8> = caps["joltage"].split(|b| *b == b',').map(|digits| digits[0] - b'0').collect();

        let buttons: Vec<Vec<usize>> = caps["buttons"]
            .split(|b| *b == b' ')
            .map(|button_str| {
                button_str[1..button_str.len() - 1]
                    .split(|b| *b == b',')
                    .map(|digits| (digits[0] - b'0') as usize)
                    .collect()
            })
            .collect();

        let mut prev_presses: Vec<Vec<usize>> = vec![vec![]; buttons.len()];

        for n in 1.. {
            let mut next_presses = vec![];
            for prev in &prev_presses {
                for x in 0..buttons.len() {
                    let next = [&prev[..], &[x]].concat();
                    let mut result = vec![0u8; machine.len()];
                    for &press in &next {
                        for &inc in &buttons[press]{
                            result[inc] += 1;
                        }
                    }
                    println!("presses {next:?}");
                    println!("result {result:?}");
                    println!("machine {machine:?}");
                    if result == machine {
                        println!("{result:?}");
                        println!("{next:?}");
                        println!("total: {n}");
                        total_presses += n;
                        continue 'machine;
                    }
                    if !result.iter().zip(&machine).any(|(a, b)| a > b){
                        next_presses.push(next);
                    }
                }
            }

            std::mem::swap(&mut prev_presses, &mut next_presses);
        }
    }

    Some(total_presses as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
