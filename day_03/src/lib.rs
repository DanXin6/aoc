use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct GroupSprite {
    pub group_1: AocSprite,
    pub group_2: AocSprite,
    pub group_3: AocSprite,
}

impl GroupSprite {
    pub fn unqiue_key(&self) -> Option<char> {
        let group_1 = self.group_1.get_different_str();
        let group_2 = self.group_2.get_different_str();
        let group_3 = self.group_3.get_different_str();

        // find the same key
        for c in group_1.chars() {
            if group_2.contains(c) && group_3.contains(c) {
                return Some(c);
            }
        }
        None
    }
}

#[derive(Debug, Clone, Default)]
pub struct AocSprite {
    same: Option<char>,
    pub group: String,
    pub members: Vec<String>,
}

impl AocSprite {
    pub fn new(group: String, members: Vec<String>) -> Self {
        AocSprite {
            same: None,
            group,
            members,
        }
    }

    pub fn get_same_char(&mut self) -> Option<char> {
        if self.same.is_some() {
            return self.same;
        }

        let part_1 = &self.members[0];
        let part_2 = &self.members[1];
        for c in part_1.chars() {
            if part_2.contains(c) {
                self.same = Some(c);
                break;
            }
        }
        self.same
    }

    pub fn get_different_str(&self) -> String {
        let part_1 = &self.members[0];
        let part_2 = &self.members[1];

        let mut str_map = HashMap::new();
        for c in part_1.chars() {
            if !part_2.contains(c) {
                str_map.insert(c, 1);
            }
        }

        for c in part_2.chars() {
            if !part_1.contains(c) {
                str_map.insert(c, 1);
            }
        }

        let mut result = String::new();
        for (k, _) in str_map {
            result.push(k);
        }

        result
    }
}

pub fn letter_to_customer_int(c: u8) -> u8 {
    if c >= 97 as u8 && c <= 122 as u8 {
        return c - 96;
    } else if c >= 65 as u8 && c <= 90 as u8 {
        return c - 38;
    } else {
        return 0;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_letter_to_customer_int() {
        assert_eq!(1, letter_to_customer_int(97)); // a
        assert_eq!(26, letter_to_customer_int(122)); // z
        assert_eq!(27, letter_to_customer_int(65)); // A
        assert_eq!(52, letter_to_customer_int(90)); // Z
    }
}
