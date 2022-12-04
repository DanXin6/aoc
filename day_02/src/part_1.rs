use std::io::BufRead;

use day_02::{Operate, file_reader};

fn main() {
    let reader = file_reader("D:\\develop\\project\\rust\\aoc\\input\\day_02.txt");
    let lines = reader.lines();

    let mut result = 0;
    for line in lines {
        let line = line.unwrap();

        let mut iter = line.split_whitespace();
        let a = Operate::from(iter.next().unwrap());
        let b = Operate::from(iter.next().unwrap());

        let r = b.compare(&a);
        result += r + b;
    }

    println!("{}", result);
}
