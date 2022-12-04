use std::io::BufRead;

use day_03::{AocSprite, GroupSprite};

fn main() {
    let reader = utils::file_reader("D:\\develop\\project\\rust\\aoc\\input\\day_03.txt");
    let lines = reader.lines();

    let mut group_vec: Vec<GroupSprite> = Vec::new();

    let mut groups = GroupSprite::default();
    for (i, line) in lines.enumerate() {
        let line = line.unwrap();
        let middle = line.len() / 2;
        let part_1 = line[..middle].to_string();
        let part_2 = line[middle..].to_string();
        let aoc = AocSprite::new(line, vec![part_1, part_2]);
        match i % 3 {
            0 => groups.group_1 = aoc,
            1 => groups.group_2 = aoc,
            2 => {
                groups.group_3 = aoc;
                group_vec.push(groups.clone());
                groups = GroupSprite::default();
            }
            _ => {}
        }
    }
    let mut result: i32 = 0;
    for group in group_vec {
        if let Some(c) = group.unqiue_key() {
            let r = day_03::letter_to_customer_int(c as u8) as i32;
            result += r;
            println!("same: {}, r: {}, result: {}", c, r, result);
        }
    }
}
