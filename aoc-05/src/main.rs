use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    // let input = include_str!("./test.txt");

    let mut rule_map: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut updates: Vec<Vec<u64>> = Vec::new();
    for line in input.lines() {
        if line.contains('|') {
            let mut split = line.split("|");
            let x = split.next().unwrap().parse().unwrap();
            let y = split.next().unwrap().parse().unwrap();

            rule_map.entry(x).or_default().push(y);
        }
        else if line.contains(',') {
            updates.push(line.split(',').map(|x| x.parse().unwrap()).collect());
        }
    }

    let mut result_1 = 0;
    let mut result_2 = 0;
    for update in updates {
        if is_correctly_ordered(&rule_map, &update) {
            let mid = update[update.len() / 2];
            // println!("Correctly ordered: {update:?}, add middle {mid}");
            result_1 += mid;
        }
        else {
            let new_order = fix_order(&rule_map, &update);
            let mid = new_order[new_order.len() / 2];
            // println!("Correctly ordered: {new_order:?}, add middle {mid}");
            result_2 += mid;
        }
    }

    println!("Part 1: {result_1}");
    println!("Part 2: {result_2}");
}

fn fix_order(rule_map: &HashMap<u64, Vec<u64>>, update: &[u64]) -> Vec<u64> {
    let mut new_order: Vec<u64> = Vec::new();

    // println!("Fix {update:?}");
    for i in 0..update.len() {
        let page = update[i];

        if let Some(rules) = rule_map.get(&page) {
            let mut position = 0;
            // println!("Rules for {page}: {rules:?}");
            for j in 0..new_order.len() {
                let p = new_order[j];
                if rules.contains(&p) {
                    // println!("Found rule. Set position to: {j}");
                    position = j;
                    break;
                }
                position += 1;
            }
            // println!("Insert {page} at position {position}");
            new_order.insert(position, page);
        }
        else {
            // println!("Insert page without rule");
            new_order.push(page);
        }

        // println!("{new_order:?}");
    }

    new_order
}

fn is_correctly_ordered(rule_map: &HashMap<u64, Vec<u64>>, update: &[u64]) -> bool {
    let mut page_set: HashSet<u64> = HashSet::new();
    for page in update.iter().rev() {
        if page_set.contains(page) {
            return false;
        }
        if let Some(rules) = rule_map.get(page) {
            for rule in rules {
                page_set.insert(*rule);
            }
        }
    }
    true
}