use std::{collections::{HashMap, HashSet}, fs};

fn main() {
    {
        let (r_rules, l_rules, page_updates) = read_input();
        let mut sum = 0;
        for page in page_updates {
            if has_correct_order(&page, &r_rules, &l_rules) { 
                sum += page[page.len()/2];
            }
        }
        println!("Part One: {:}", sum);
    }

    {
        let (r_rules, l_rules, page_updates) = read_input();
        let mut sum = 0;
        for mut page in page_updates {
            if has_correct_order(&page, &r_rules, &l_rules) { continue; }
            let mut sorted_page: Vec<i32> = vec![];
            while !page.is_empty() {
                for i in 0..page.len() {
                    let mut can_place = true;
                    for j in 0..page.len() {
                        if i == j { continue }
                        if 
                            r_rules.contains_key(&page[j]) && 
                            r_rules.get(&page[j]).unwrap().contains(&page[i]) {
                            can_place = false;
                            break;
                        }
                    }

                    if can_place {
                        sorted_page.push(page[i]);
                        page.remove(i);
                        break;
                    }
                }
            }

            sum += sorted_page[sorted_page.len()/2];
        }
        println!("Part Two: {:}", sum);
    }
}

fn read_input() -> (HashMap<i32, HashSet<i32>>, HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    const INPUT_PATH: &str = "src/input.txt";
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    let input: Vec<&str> = input.split("\n\n").collect();

    let mut r_ordering_rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut l_ordering_rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    for x in input[0].split("\n") {
        let rule: Vec<i32> = x.split("|").map(|x| x.parse::<i32>().unwrap()).collect();
        
        r_ordering_rules
            .entry(rule[0])
            .or_insert(HashSet::new())
            .insert(rule[1]);

        l_ordering_rules
            .entry(rule[1])
            .or_insert(HashSet::new())
            .insert(rule[0]);
    }

    let mut page_updates: Vec<Vec<i32>> = vec![];
    for x in input[1].split("\n") {
        let pages:Vec<i32> = x.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        page_updates.push(pages);
    }

    (r_ordering_rules, l_ordering_rules, page_updates)
}

fn has_correct_order(page: &Vec<i32>, r_rules: &HashMap<i32, HashSet<i32>>, l_rules: &HashMap<i32, HashSet<i32>>) -> bool {
    let mut is_valid = true;

    for i in 0..page.len() {
        let right_hand_updates: &HashSet<i32> = &page[i+1..].into_iter().cloned().collect(); 
        let left_hand_updates: &HashSet<i32> = &page[..i].into_iter().cloned().collect();        
        if 
            (r_rules.contains_key(&page[i]) &&
            !right_hand_updates.is_subset(&r_rules.get(&page[i]).unwrap())) ||
            (l_rules.contains_key(&page[i]) &&
            !left_hand_updates.is_subset(&l_rules.get(&page[i]).unwrap()))
        {
            is_valid = false;
            break;
        }
    }

    is_valid
}