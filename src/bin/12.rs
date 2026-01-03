advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    for line in input.lines().take(30) {}
    let  mut dismiss = 0;
    for line in input.lines().skip(30) {
        let (region_size, quantities) = line.split_once(": ")?;
        let (width, length) = region_size.split_once('x')?;
        let area = width.parse::<u64>().unwrap() * length.parse::<u64>().unwrap();
        let total_size: u64 = quantities
            .split(' ')
            .map(|quantity| quantity.parse::<u64>().unwrap())
            .enumerate()
            .map(|(i, quantity)| {
                quantity
                    * match i {
                        0 | 1 | 3 | 5 => 7,
                        2 => 6,
                        4 => 5,
                        _ => panic!(),
                    }
            }).sum();

        let square_size: u64 = quantities
            .split(' ')
            .map(|quantity| quantity.parse::<u64>().unwrap())
            .enumerate()
            .map(|(i, quantity)| {
                quantity
                    * 9
            }).sum();
        println!("{area} {total_size} {square_size}");
        if total_size > area || square_size <= area {dismiss += 1}
    }
    println!("{}", input.lines().skip(30).count()); // 1000
    println!("{dismiss}"); //450 !!!
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
