use std::fs;

fn main() {
    {
        const TARGET_WORDS: [&str; 2] = ["XMAS", "SAMX"];
    
        let input = read_input();
        let mut count = 0;
        for i in 0..input.len() {
            for j in 0..input[i].len() {
                // Check horizontal
                if j + 3 < input[i].len() {
                    let word = input[i][j..j+4].join("");
                    if TARGET_WORDS.contains(&word.as_str()) { 
                        count += 1; 
                    }
                }
    
                // Check vertical
                if i + 3 < input.len() {
                    let word = [
                        input[i][j].as_str(), 
                        input[i+1][j].as_str(),
                        input[i+2][j].as_str(),
                        input[i+3][j].as_str(),
                    ].join("");
                    if TARGET_WORDS.contains(&word.as_str()) { 
                        count += 1; 
                    }
                }
    
                // Check diagonal (left to right)
                if i + 3 < input.len() && j + 3 < input[i].len() {
                    let word = [
                        input[i][j].as_str(), 
                        input[i+1][j+1].as_str(),
                        input[i+2][j+2].as_str(),
                        input[i+3][j+3].as_str(),
                    ].join("");
                    if TARGET_WORDS.contains(&word.as_str()) { 
                        count += 1; 
                    }
                }
    
                // Check diagonal (left to right)
                if i + 3 < input.len() && j > 2 {
                    let word = [
                        input[i][j].as_str(), 
                        input[i+1][j-1].as_str(),
                        input[i+2][j-2].as_str(),
                        input[i+3][j-3].as_str(),
                    ].join("");
                    if TARGET_WORDS.contains(&word.as_str()) { 
                        count += 1; 
                    }
                }
            }
        }
    
        println!("Part One: {}", count);
    }

    {
        const TARGET_WORDS: [&str; 2] = ["MAS", "SAM"];
    
        let input = read_input();
        let mut count = 0;

        for i in 0..input.len() {
            for j in 0..input[i].len() {

                if !(j + 2 < input[i].len() && i + 2 < input.len()) {
                    continue;
                }
            
                let one = [
                    input[i][j].as_str(), 
                    input[i+1][j+1].as_str(),
                    input[i+2][j+2].as_str(),
                ].join("");

                let two = [
                    input[i][j+2].as_str(), 
                    input[i+1][(j+2)-1].as_str(),
                    input[i+2][(j+2)-2].as_str(),
                ].join("");

                if TARGET_WORDS.contains(&one.as_str()) && TARGET_WORDS.contains(&two.as_str()) {
                    count += 1;
                }
            }
        }

        println!("Part Two: {}", count);
    }
}

fn read_input() -> Vec<Vec<String>> {
    const INPUT_PATH: &str = "src/input.txt";
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    let input: Vec<&str> = input.split("\n").collect::<Vec<&str>>();

    let mut matrix: Vec<Vec<String>> = vec![];
    for x in input.iter() {
        let pairs = x.split("")
            .map(|y| y.to_string())
            .filter(|x| !x.is_empty())
            .collect::<Vec<String>>();
        matrix.push(pairs);
    }

    matrix
}
