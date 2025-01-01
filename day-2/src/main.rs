use std::fs;

fn main() {

    // Pat One
    {
        let reports = read_input();
        let mut safe_locations = 0;
        for report in reports.iter() {
            if is_safe(report) {
                safe_locations += 1;
            }
        }
        println!("Part One: {:?}", safe_locations);
    }

    // Part Two
    {
        let reports = read_input();
        let mut safe_locations = 0;

        for report in reports.iter() {
            if is_safe(report) {
                safe_locations += 1;
            }
            else {
                for i in 0..report.len() {
                    if is_safe(&[&report[..i], &report[i + 1..]].concat()) {
                        safe_locations += 1;
                        break;
                    }
                }
            };
        }
        println!("Part Two: {:?}", safe_locations);
    }
}

fn read_input() -> Vec<Vec<i32>> {
    const INPUT_PATH: &str = "src/input.txt";
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    let input: Vec<&str> = input.split("\n").collect::<Vec<&str>>();

    let mut levels: Vec<Vec<i32>> = vec![];
    for x in input.iter() {
        let pairs = x.split(" ")
            .map(|y| y.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        levels.push(pairs);
    }

    levels
}

fn is_safe(report: &Vec<i32>) -> bool {
    let mut is_safe = true;
    let is_decreasing = (report[0] - report[1]).is_positive();
    let is_increasing = (report[0] - report[1]).is_negative();

    for y in 0..(report.len() - 1) {
        let distance = report[y] - report[y + 1];

        if 
        (is_decreasing && distance.is_negative()) || 
        (is_increasing && distance.is_positive()) || 
        !(1..=3).contains(&(distance.abs())) {
            is_safe = false;
            break;
        }
    }

    is_safe
}