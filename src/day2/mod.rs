pub fn day_two_pt_one() {
    let pairs = parse_file();
    let mut invalid_ids: Vec<i64> = vec![];

    for (start, end) in pairs {
        let mut cur_idx = start;

        while cur_idx <= end {
            if is_invalid_rule1(cur_idx) { 
                invalid_ids.push(cur_idx);
            }

            cur_idx += 1;
        }
    }

    println!("{}", invalid_ids.iter().sum::<i64>());
}

pub fn day_two_pt_two() {
    let pairs = parse_file();
    let mut invalid_ids: Vec<i64> = vec![];

    for (start, end) in pairs {
        let mut cur_idx = start;

        while cur_idx <= end {
            if is_invalid_rule2(cur_idx) {
                invalid_ids.push(cur_idx);
            }

            cur_idx += 1;
        }
    }

    println!("{}", invalid_ids.iter().sum::<i64>());
}

fn parse_file() -> Vec<(i64, i64)> {
    std::fs::read_to_string("src/day2/input")
        .unwrap()
        .trim_ascii()
        .split(",")
        .map(|id| {
            let (first, last) = id.split_once("-").expect("Invalid input, no hyphen in ID");

            let first: i64 = first.parse().unwrap();
            let last: i64 = last.parse().unwrap();

            (first, last)
        })
        .collect::<Vec<(i64, i64)>>()
}


/// Determines if an ID is invalid if an id is repeated twice
///
/// Because it this one is ONLY twice, we only care about IDs with a 
/// len divisible by two (symmetrical).
///
/// From here we just need to check if each half is equal.
fn is_invalid_rule1(num: i64) -> bool {
    let chars = num.to_string().chars().collect::<Vec<char>>();
    let len = chars.len();

    // we only care about lengths divisble by 2 because we need to mirror the sides
    if len % 2 == 0 {
        let (h1, h2) = chars.split_at(len / 2);
        let h1 = h1.iter().collect::<String>();
        let h2 = h2.iter().collect::<String>();

        if h1 == h2 {
            return true;
        }
    }

    return false;
}

/// Determines only from the beginning, if any value is repeated at least twice.
///
/// We do this by determining the length of the number, iterating over that length
/// and chunking by that length. All we have to do then is determine if the Vec is full
/// of equal parts.
fn is_invalid_rule2(num: i64) -> bool {
    let pairs = num.to_string().chars().collect::<Vec<char>>();
    let len = pairs.len();

    for i in 0..len {
        let chunks = pairs
            .chunks(i + 1)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>();

        let invalid = has_equal_parts(&chunks);

        if invalid {
            return true;
        }
    }

    return false;
}

fn has_equal_parts(input: &Vec<String>) -> bool {
    if input.len() == 1 { return false; }
    let mut part: Option<String> = None;

    for i in input.iter() {
        if let Some(ref p) = part {
            if i != p {
                return false
            }
        } else {
            part = Some(i.clone());
        }
    }
    
    return true;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pairs_test() {
        let result = is_invalid_rule2(12341234);
        assert_eq!(result, true);

        let result = is_invalid_rule2(123123123);
        assert_eq!(result, true);

        let result = is_invalid_rule2(1212121212);
        assert_eq!(result, true);

        let result = is_invalid_rule2(1111111);
        assert_eq!(result, true);
    }
}
