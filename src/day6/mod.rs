use std::str::FromStr;

#[derive(Debug)]
enum Operation {
    Addition,
    Multiplication
}

const ROW_HEIGHT: usize = 4;

#[derive(Debug)]
struct Problem {
    inputs: Vec<i64>,
    opp: Operation
}

#[derive(Debug)]
struct OperationError;

impl FromStr for Operation {
    type Err = OperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Addition),
            "*" => Ok(Operation::Multiplication),
            _ => Err(OperationError)
        }
    }
}

pub fn day_six_pt_one() {
    let input = std::fs::read_to_string("src/day6/input").unwrap()
        .lines()
        .map(|l| 
            l.split(" ")
                .filter_map(|li| {
                    match li {
                        "" => None,
                        _ => Some(li.trim().to_string())
                    }
                })
                .collect::<Vec<String>>()

        )
        .collect::<Vec<Vec<String>>>();

    let input = pivot(input);
    let problems = parse(input);

    let sum: i64 = problems
        .iter()
        .map(|p| {
            match p.opp {
                Operation::Addition => p.inputs.iter().sum::<i64>(),
                Operation::Multiplication => p.inputs.iter().product::<i64>()
            }
        })
        .sum();

    println!("{:?}", sum);
}

pub fn day_six_pt_two() {
    let input = std::fs::read_to_string("src/day6/input").unwrap()
        .lines()
        .map(|l| 
            l.chars().collect::<Vec<char>>()
        )
        .collect::<Vec<Vec<char>>>();

    // pivot the input
    let input = pivot(input);

    let (chunk_idxs, _): (Vec<usize>, Vec<_>) = input
        .iter()
        .enumerate()
        .filter(|(_, cs)| {
            cs.iter().all(|c| *c == ' ')
        })
        .collect();

    let mut chunks = vec![];

    // chunk input
    for i in 0..chunk_idxs.iter().len() {
        let current = chunk_idxs[i];

        match i.checked_sub(1) {
            Some(idx) => {
                let prev = chunk_idxs[idx];
                let chunk = input[prev + 1..current].to_vec();
                chunks.push(chunk);
            },
            None => {
                let chunk = input[0..current].to_vec();
                chunks.push(chunk);
            }
        }
    }

    chunks.reverse();

    let mut sum = 0;

    for chunk in chunks.iter_mut() {
        let mut operation = None;

        for chars in chunk.iter_mut() {
            if let Some(opp_char) = chars.get(ROW_HEIGHT) {
                if let Ok(opp) = opp_char.to_string().parse::<Operation>() {
                    operation = Some(opp);
                    chars[ROW_HEIGHT] = ' ';
                }
            }
        }

        chunk.reverse();

        println!("chunk: {:?}", chunk);

        let num_chunks = chunk
            .iter()
            .map(|c| c.iter().collect::<String>().trim().to_string().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        println!("{:?}", num_chunks);

        if let Some(opp) = operation {
            let res = match opp {
                Operation::Addition => num_chunks.iter().sum::<i64>(),
                Operation::Multiplication => num_chunks.iter().product::<i64>()
            };

            println!("{}", res);

            sum += res;
        }
    }

    println!("sum: {}", sum);
}

fn pivot<T: Clone>(input: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut pivoted: Vec<Vec<T>> = Vec::new();

    for row in &input {
        for (col_idx, item) in row.iter().enumerate() {
            if col_idx >= pivoted.len() {
                pivoted.push(Vec::new());
            }
            pivoted[col_idx].push(item.clone());
        }
    }

    pivoted
}

fn parse(input: Vec<Vec<String>>) -> Vec<Problem> {
    let mut problems = vec![];

    for r in input.into_iter() {
        let mut nums = vec![];
        let mut operation = None;

        for i in r.into_iter() {
            match i.parse::<i64>() {
                Ok(n) => nums.push(n),
                Err(_) => {
                    operation = Some(i.parse::<Operation>().unwrap());
                }
            }
        }

        problems.push(Problem { inputs: nums, opp: operation.unwrap() })
    }

    problems
}
