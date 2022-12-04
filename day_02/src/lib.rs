use std::ops::Add;

#[derive(Debug, Clone)]
pub enum Operate {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

pub enum R {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

pub enum SpecifyR {
    Win,
    Lose,
    Draw,
}

impl Add<Operate> for R {
    type Output = i32;

    fn add(self, rhs: Operate) -> Self::Output {
        self as i32 + rhs as i32
    }
}

impl From<&str> for Operate {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Operate::Rock,
            "B" | "Y" => Operate::Paper,
            "C" | "Z" => Operate::Scissor,
            _ => panic!("Unknown operate"),
        }
    }
}

impl Into<i32> for Operate {
    fn into(self) -> i32 {
        match self {
            Operate::Rock => 1,
            Operate::Paper => 2,
            Operate::Scissor => 3,
        }
    }
}

impl Operate {
    pub fn compare(&self, other: &Operate) -> R {
        match (self, other) {
            (Operate::Rock, Operate::Paper) => R::Lose,
            (Operate::Rock, Operate::Scissor) => R::Win,
            (Operate::Paper, Operate::Rock) => R::Win,
            (Operate::Paper, Operate::Scissor) => R::Lose,
            (Operate::Scissor, Operate::Rock) => R::Lose,
            (Operate::Scissor, Operate::Paper) => R::Win,
            _ => R::Draw,
        }
    }

    pub fn specify(&self, r: &SpecifyR) -> Self {
        match r {
            SpecifyR::Win => match self {
                Operate::Rock => Operate::Paper,
                Operate::Paper => Operate::Scissor,
                Operate::Scissor => Operate::Rock,
            },
            SpecifyR::Lose => match self {
                Operate::Rock => Operate::Scissor,
                Operate::Paper => Operate::Rock,
                Operate::Scissor => Operate::Paper,
            },
            SpecifyR::Draw => self.clone(),
        }
    }
}


impl From::<&str> for SpecifyR {
    fn from(value: &str) -> Self {
        match value {
            "Z" => SpecifyR::Win,
            "X" => SpecifyR::Lose,
            "Y" => SpecifyR::Draw,
            _ => panic!("Unknown specify r"),
        }
    }
}

impl From::<&SpecifyR> for R {
    fn from(value: &SpecifyR) -> Self {
        match value {
            SpecifyR::Win => R::Win,
            SpecifyR::Lose => R::Lose,
            SpecifyR::Draw => R::Draw,
        }
    }
}

pub fn file_reader(path: &str) -> std::io::BufReader<std::fs::File> {
    let file = std::fs::File::open(path).unwrap();
    return std::io::BufReader::new(file);
}