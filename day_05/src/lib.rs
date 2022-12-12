#[derive(Debug, Clone)]
pub struct SpriteCrate {
    pub crate_stack: Vec<CrateStack>,
}

impl From<String> for SpriteCrate {
    fn from(value: String) -> Self {
        let mut crate_stack = Vec::new();
        let mut stack = Vec::new();

        let line = value.split("\n").collect::<Vec<&str>>();

        let max_length = line.iter().map(|x| x.len()).max().unwrap();
        for i in 0..max_length {
            let mut s = String::new();
            for j in 0..line.len() {
                if i < line[j].len() {
                    s.push(line[j].chars().nth(i).unwrap());
                } else {
                    s.push(' ');
                }
            }
            stack.push(s);
        }

        let mut result = Vec::new();
        stack.iter_mut().for_each(|x| {
            x.chars().last().map(|c| {
                if c.is_ascii_digit() {
                    let mut x = x.clone();
                    x.remove(x.len() - 1);
                    result.push(x);
                }
            });
        });

        result.iter().for_each(|x| {
            crate_stack.push(CrateStack {
                stack: x.trim().to_string().clone(),
            });
        });

        SpriteCrate { crate_stack }
    }
}

impl SpriteCrate {
    pub fn transfer(&mut self, moves: Vec<Move>, rev: bool) {
        moves.iter().for_each(|x| {
            if self.length() < x.from || self.length() < x.to {
                panic!("invalid move");
            }
            // 移动对象
            let mut o = String::new();
            if self.crate_stack[x.from].stack.len() < x.number {
                o = self.crate_stack[x.from].stack.clone();
            } else {
                o = self.crate_stack[x.from].stack[..x.number].to_string();
            }

            // 将移动对象添加到目标
            if rev {
                o = reverse(o);
            }
            
            self.crate_stack[x.to].stack.insert_str(0, &o);
            // 将移动对象从原位置删除
            self.crate_stack[x.from].stack = self.crate_stack[x.from].stack[o.len()..].to_string();

            println!(
                "move {} ==> from: {}, to: {}",
                o, self.crate_stack[x.from].stack, self.crate_stack[x.to].stack
            );
        });
    }

    pub fn last(&self) -> String {
        let mut s = String::new();
        self.crate_stack.iter().for_each(|x| {
            let mut x = x.clone();
            s.push(x.stack.remove(0));
        });

        s
    }

    pub fn length(&self) -> usize {
        self.crate_stack.len()
    }
}

#[derive(Debug, Clone)]
pub struct CrateStack {
    pub stack: String,
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub number: usize,
    pub from: usize,
    pub to: usize,
}

impl From<String> for Move {
    fn from(value: String) -> Self {
        let ss: Vec<&str> = value.split(" ").collect();
        if ss.len() != 6 {
            panic!("invalid input: {}", value);
        }

        let number = ss[1].parse::<usize>().unwrap();
        let from = ss[3].parse::<usize>().unwrap() - 1;
        let to = ss[5].parse::<usize>().unwrap() - 1;

        Move { number, from, to }
    }
}

pub fn reverse(input: String) -> String {
    let mut s = String::new();
    input.chars().rev().for_each(|x| {
        s.push(x);
    });

    s
}