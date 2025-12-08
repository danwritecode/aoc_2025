type Lower = i64;
type Upper = i64;

struct Ingredients {
    fresh_ranges: Vec<(Lower, Upper)>,
    ingredient_ids: Vec<i64>
}

pub fn day_five_pt_one() {
    let ingredients = parse_file();

    let fresh_ingredient_ct = ingredients.ingredient_ids
        .into_iter()
        .filter(|id| {
            let ct = ingredients.fresh_ranges
                .iter()
                .filter(|(lower, _)| id >= lower)
                .filter(|(_, upper)| id <= upper)
                .count();

            ct > 0
        })
        .count();


    println!("{}", fresh_ingredient_ct);
}

pub fn day_five_pt_two() {
    let ingredients = parse_file();

    for (lower, upper) in ingredients.fresh_ranges.iter() {
        let ranges = ingredients.fresh_ranges
            .iter()
            .filter(|(l, _)| lower >= l)
            .filter(|(_, u)| upper <= u)
            .collect::<Vec<_>>();

        println!("{:?}", ranges);
    }
}

fn parse_file() -> Ingredients {
    let input = std::fs::read_to_string("src/day5/input")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let (split_idx, _) = input.iter().enumerate().find(|l| l.1 == "").unwrap();
    let input = input.into_iter().filter(|l| l != "").collect::<Vec<String>>();

    let (fresh_ranges, ingredient_ids) = input.split_at(split_idx);

    let ingredient_ids: Vec<i64> = ingredient_ids.into_iter().map(|i| i.parse::<i64>().unwrap()).collect();

    let fresh_ranges = fresh_ranges
        .iter()
        .map(|r| {
            let parts = r.split("-").collect::<Vec<&str>>();

            let lower = parts[0].parse::<i64>().unwrap();
            let upper = parts[1].parse::<i64>().unwrap();

            (lower, upper)
        })
        .collect::<Vec<(i64, i64)>>();

    Ingredients { fresh_ranges, ingredient_ids }
}
