use std::{collections::HashMap, fs, vec};

fn main() {

    // Part One
    {
        let (left, right) = read_input();
        let mut sum = 0;
        for i in 0..left.len() {
            sum += (left[i] - right[i]).abs();
        }
        println!("Part One: {}", sum);

    }
    
    // Part Two
    {
        let mut scores: HashMap<i32, i32> = HashMap::new();
        let (left, right) = read_input();
        let mut sum = 0;
        for x in 0..left.len() {
            if !scores.contains_key(&left[x]) {
                let mut score = 0;
                for y in 0..right.len() {
                    if left[x] == right[y] {
                        score += 1;
                    }
                }
                scores.insert(left[x], left[x] * score);
            }
            sum += scores.get(&left[x]).unwrap(); 
        }
        println!("Part Two: {}", sum);
    }
}

fn read_input() -> (Vec<i32>, Vec<i32>) {
    const INPUT_PATH: &str = "src/input.txt";
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    let input: Vec<&str> = input.split("\n").collect::<Vec<&str>>();

    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];
    for x in input.iter() {
        let pairs = x.split("   ")
            .map(|y| y.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        left_list.push(pairs[0]);
        right_list.push(pairs[1]);
    }

    left_list.sort();
    right_list.sort();

    (left_list, right_list)
}