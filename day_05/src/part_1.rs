use std::io::BufRead;

use day_05::{Move, SpriteCrate};

fn main() {
    let reader = utils::file_reader("D:\\develop\\project\\rust\\aoc\\input\\day_05.txt");
    let lines = reader.lines();

    let mut crate_parts = Vec::new();
    let mut moves = Vec::new();
    let mut finished = false;

    for l in lines {
        let line = l.unwrap();
        if line.is_empty() {
            finished = true;
            continue;
        }

        if !finished {
            crate_parts.push(line);
        } else {
            moves.push(Move::from(line));
        }
    }

    let crate_parts = crate_parts.join("\n");
    let mut sprite_crate = SpriteCrate::from(crate_parts);

    sprite_crate.transfer(moves, true);

    println!("{}", sprite_crate.last());
}
