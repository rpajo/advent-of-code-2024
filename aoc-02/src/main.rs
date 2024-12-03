fn main() {
    let input = include_str!("./input.txt");

    let mut valid_reports = 0;
    let mut valid_reports_corrected = 0;
    for line in input.lines() {
        let mut report = line
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if report[report.len() - 1] < report[0] {
            report.reverse();
        }
        let mut report_iter = report.iter().peekable();

        let mut is_valid = true;
        let mut can_dampen = true;
        let mut a = *report_iter.next().unwrap();
        let mut prev = 0;
        while let Some(&b) = report_iter.next() {
            let c = report_iter.peek();
            if is_valid_pair(a, b) {
                prev = a;
                a = b;
                continue;
            }
            else if can_dampen {
                can_dampen = false;

                if prev > 0 && is_valid_pair(prev, b) {
                    a = b;
                }
                else if c.is_some() && is_valid_pair(a, **c.unwrap()) {
                    a = *report_iter.next().unwrap();
                }
                else if prev == 0 || c.is_none() {
                    a = b;
                }
                else {
                    is_valid = false;
                    break;
                }
            }
            else {
                is_valid = false;
                break;
            }
        }

        if is_valid && can_dampen {
            valid_reports += 1;
            valid_reports_corrected += 1;
        } else if is_valid && !can_dampen {
            valid_reports_corrected += 1;
        }
    }

    println!("Part 1: {valid_reports}");
    println!("Part 1: {valid_reports_corrected}");
}

fn is_valid_pair(a: i32, b: i32) -> bool {
    a < b && b - a <= 3
}