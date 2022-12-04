use std::io::BufRead;

use day_03::letter_to_customer_int;

fn main() {
    let reader = utils::file_reader("D:\\develop\\project\\rust\\aoc\\input\\day_03.txt");
    let lines = reader.lines();

    let mut result: i32 = 0;
    for (i, line) in lines.enumerate() {
        let line = line.unwrap();

        let middle = line.len() / 2;

        let part_1 = line[..middle].to_string();
        let part_2 = line[middle..].to_string();

        // find the same char
        for c in part_1.chars() {
            if part_2.contains(c) {
                let r = letter_to_customer_int(c as u8) as i32;
                result += r;
                println!("line: {}, part_1: {}, part_2: {}, same: {}, r: {}, result: {}", i, part_1, part_2, c, r, result);
                break;
            }
        }
    }

    println!("{}", result);
}
