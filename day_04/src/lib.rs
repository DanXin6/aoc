pub struct GroupSprite {
    pub group_1: AocSprite,
    pub group_2: AocSprite,
}

impl GroupSprite {
    pub fn new(group_1: AocSprite, group_2: AocSprite) -> Self {
        GroupSprite { group_1, group_2 }
    }

    pub fn is_fully_contains(&self) -> bool {
        if self.group_1.left_bound >= self.group_2.left_bound
            && self.group_1.right_bound <= self.group_2.right_bound
        {
            return true;
        }

        if self.group_2.left_bound >= self.group_1.left_bound
            && self.group_2.right_bound <= self.group_1.right_bound
        {
            return true;
        }

        false
    }

    pub fn is_partial_overlap(&self) -> bool {
        if self.group_1.left_bound >= self.group_2.left_bound
            && self.group_1.left_bound <= self.group_2.right_bound
        {
            return true;
        }

        if self.group_1.right_bound >= self.group_2.left_bound
            && self.group_1.right_bound <= self.group_2.right_bound
        {
            return true;
        }

        if self.group_2.left_bound >= self.group_1.left_bound
            && self.group_2.left_bound <= self.group_1.right_bound
        {
            return true;
        }

        if self.group_2.right_bound >= self.group_1.left_bound
            && self.group_2.right_bound <= self.group_1.right_bound
        {
            return true;
        }

        false
    }
}

pub struct AocSprite {
    pub left_bound: i32,
    pub right_bound: i32,
}

impl From<String> for AocSprite {
    fn from(value: String) -> Self {
        let ss: Vec<&str> = value.split("-").collect();
        if ss.len() != 2 {
            panic!("invalid input: {}", value);
        }

        let left_bound = ss[0].parse::<i32>().unwrap();
        let right_bound = ss[1].parse::<i32>().unwrap();

        AocSprite::new(left_bound, right_bound)
    }
}
impl AocSprite {
    pub fn new(left_bound: i32, right_bound: i32) -> Self {
        AocSprite {
            left_bound,
            right_bound,
        }
    }
}
