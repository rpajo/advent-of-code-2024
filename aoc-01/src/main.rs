use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let (mut first, mut second): (Vec<i64>, Vec<i64>) = input
        .lines()
        .map(|line| line.split("   "))
        .map(|mut elements| {
            (
                elements.next().unwrap().parse::<i64>().unwrap(),
                elements.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .unzip();

    first.sort();
    second.sort();

    let result_1 = first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i64>();

    println!("Part 1: {}", result_1);

    let mut occurances = HashMap::new();
    for n in second {
        *occurances.entry(n).or_insert(0) += 1;
    }
    let mut result_2 = 0;
    for n in first {
        result_2 += n * occurances.get(&n).unwrap_or(&0);
    }

    println!("Part 2: {}", result_2);
}
