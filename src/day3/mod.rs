const RADIX: u32 = 10;

#[derive(Debug)]
struct Largest {
    found_idx: usize,
    value: u32
}

pub fn day_three_pt_one() { 
    let input = parse_input();

    let total: u32 = input
        .iter()
        .map(|r| {
            let fl = find_largest(&r, 0, 1);
            let sl = find_largest(&r, fl.found_idx + 1, 0);
            let total = format!("{}{}", fl.value, sl.value).parse::<u32>().unwrap();

            total
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum();

    println!("{}", total);
}

pub fn day_three_pt_two() { 
    let input = parse_input();

    let total: u32 = input
        .iter()
        .map(|r| {
            let fl = find_largest(&r, 0, 1);
            let sl = find_largest(&r, fl.found_idx + 1, 0);
            let total = format!("{}{}", fl.value, sl.value).parse::<u32>().unwrap();

            total
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum();

    println!("{}", total);
}

fn parse_input() -> Vec<Vec<u32>> {
    std::fs::read_to_string("src/day3/input")
        .unwrap()
        .lines()
        .map(|l| l.to_owned().chars().map(|c| c.to_digit(RADIX).unwrap()).collect::<Vec<u32>>())
        .collect::<Vec<_>>()
}

fn find_largest(nums: &Vec<u32>, starting_idx: usize, ignore_end_size: usize) -> Largest {
    let mut largest = 0;
    let mut largest_idx = 0;
    let take = nums.len() - starting_idx - ignore_end_size;

    for (i, n) in nums.iter().enumerate().skip(starting_idx).take(take) {
        if *n > largest {
            largest = *n;
            largest_idx = i;
        }
    }

    Largest { found_idx: largest_idx, value: largest }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finding_largest_test_1() {
        let set = vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1];

        let result = find_largest(&set.clone(), 0, 1);
        assert_eq!(result.value, 9);

        let result = find_largest(&set, result.found_idx + 1, 0);
        assert_eq!(result.value, 8);
    }

    #[test]
    fn finding_largest_test_2() {
        let set = vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9];

        let result = find_largest(&set.clone(), 0, 1);
        assert_eq!(result.value, 8);

        let result = find_largest(&set, result.found_idx + 1, 0);
        assert_eq!(result.value, 9);
    }

    #[test]
    fn finding_largest_test_3() {
        let set = vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8];

        let result = find_largest(&set.clone(), 0, 1);
        assert_eq!(result.value, 7);

        let result = find_largest(&set, result.found_idx + 1, 0);
        assert_eq!(result.value, 8);
    }

    #[test]
    fn finding_largest_test_4() {
        let set = vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1];

        let result = find_largest(&set.clone(), 0, 1);
        assert_eq!(result.value, 9);

        let result = find_largest(&set, result.found_idx + 1, 0);
        assert_eq!(result.value, 2);
    }

    #[test]
    fn finding_largest_v2_test_1() {
        let set = vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1];

        let result = find_largest(&set.clone(), 0, 1);
    }

    #[test]
    fn finding_largest_v2_test_2() {
        let set = vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9];

        let result = find_largest(&set.clone(), 0, 1);
    }

    #[test]
    fn finding_largest_v2_test_3() {
        let set = vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8];

        let result = find_largest(&set.clone(), 0, 1);
    }

    #[test]
    fn finding_largest_v2_test_4() {
        let set = vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1];

        let result = find_largest(&set.clone(), 0, 1);
    }
}
