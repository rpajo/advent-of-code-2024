use regex::Regex;

#[derive(Debug)]
enum Operation {
    MulOp(i64, i64),
    EnableDisable(bool)
}

fn main() {
    let input = include_str!("./input.txt");

    let re = Regex::new(r"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))").unwrap();
    let operations: Vec<Operation> = re.captures_iter(input).map(|c| {
        let group = c.get(0).unwrap().as_str();
        if group == "do()" {
            Operation::EnableDisable(true)
        } else if group == "don't()" {
            Operation::EnableDisable(false)
        } else {
           let a = c.get(2).unwrap().as_str();
           let b = c.get(3).unwrap().as_str();
           Operation::MulOp(
                a.parse::<i64>().unwrap(), 
                b.parse::<i64>().unwrap() 
            )
        }
    })
    .collect();

    let mut part_1: i64 = 0;
    let mut part_2: i64 = 0;
    let mut mul_enabled = true;
    for op in operations {
        match op {
            Operation::MulOp(a, b) => {
                let c = a * b;
                part_1 += c;
                if mul_enabled { part_2 += c; }
            }
            Operation::EnableDisable(enabled) => {
                mul_enabled = enabled;
            }
        };
    }

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
