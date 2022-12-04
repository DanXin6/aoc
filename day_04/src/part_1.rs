use std::io::BufRead;

use day_04::{AocSprite, GroupSprite};

fn main() {
    let reader = utils::file_reader("D:\\develop\\project\\rust\\aoc\\input\\day_04.txt");
    let lines = reader.lines();

    let mut result = 0;
    for line in lines {
        let line = line.unwrap();

        // spit the line into two parts
        let parts = line.split(",").collect::<Vec<&str>>();
        if parts.len() != 2 {
            panic!("invalid input: {}", line);
        }

        // build aoc sprite
        let aoc_sprite_1 = AocSprite::from(parts[0].to_string());
        let aoc_sprite_2 = AocSprite::from(parts[1].to_string());

        // build group sprite
        let group_sprite = GroupSprite::new(aoc_sprite_1, aoc_sprite_2);

        // judge whether is the fully contains
        if group_sprite.is_fully_contains() {
            result += 1;
            println!("{} is fully contains", line);
        }
    }

    println!("{}", result);
}
