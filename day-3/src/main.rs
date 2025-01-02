use regex::Regex;
use std::fs;


fn main() {
    {
        const INPUT_PATH: &str = "src/input.txt";
        let input = fs::read_to_string(INPUT_PATH).unwrap();
        let regex= Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

        let mut sum = 0;
        for (_, [x, y]) in regex.captures_iter(&input).map(|x| x.extract()) {
            sum += x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap();
        }

        println!("Part One: {}", sum);
    }

    {
        const INPUT_PATH: &str = "src/input.txt";
        let input = fs::read_to_string(INPUT_PATH).unwrap();
        let regex= Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|(do\(\))|(don\'t\(\))").unwrap();

        let mut sum: i32 = 0;
        let mut can_multiply = true;
        for caps in regex.captures_iter(&input) {
            if let Some(x) = caps.get(1) {
                if !can_multiply {
                    continue;
                }

                let x = x.as_str().parse::<i32>().unwrap();
                let y = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
                sum += x * y;
            } 
            else if let Some(_) = caps.get(3) {
                can_multiply = true;
            } 
            else if let Some(_) = caps.get(4) {
                can_multiply = false;
            }
        }
        println!("Part Two: {}", sum);
    }
}
