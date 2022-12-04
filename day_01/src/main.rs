use std::{fs, io::BufRead};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Calories(i64);

fn main() {
    let data = fs::File::open("D:\\develop\\project\\rust\\aoc\\input\\day_01.txt").unwrap();
    let reader = std::io::BufReader::new(data);

    let mut calories = Vec::new();
    let mut current_calories: i64 = 0;

    let iter = reader.lines();
    for line in iter {
        let line = line.unwrap();

        if line != "" {
            current_calories += line.parse::<i64>().unwrap();
        } else {
            calories.push(Calories(current_calories as i64));
            current_calories = 0;
        }
    }
    calories.sort_by(|a, b| b.cmp(a));

    let mut result = 0;

    for (i, c) in calories.iter().enumerate() {
        if i > 2 {
            break;
        }
        println!("{:?}: {:?}", i, c);
        result += c.0;
    }
    
    print!("Result: {}", result);
}