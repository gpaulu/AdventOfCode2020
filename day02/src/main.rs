struct PasswordPolicy<'a> {
    pub mandated_char: char,
    // I feel that min/max names don't specify inclusive range
    pub at_least: i32,
    pub at_most: i32,
    pub password: &'a str,
}

impl<'a> PasswordPolicy<'a> {
    fn is_valid(&self) -> bool {
        let mandated_char_count = self.password.matches(self.mandated_char).count();
        mandated_char_count >= self.at_least as usize
            && mandated_char_count <= self.at_most as usize
    }
}

fn main() {
    println!("Hello, world!");
}

fn num_passwords_valid(passwords: &str) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        assert_eq!(num_passwords_valid(input), 2);
    }

    #[test]
    fn policy() {
        let policy1 = PasswordPolicy {
            mandated_char: 'a',
            at_least: 1,
            at_most: 3,
            password: "abcde",
        };
        assert_eq!(policy1.is_valid(), true);
        let policy2 = PasswordPolicy {
            mandated_char: 'b',
            at_least: 1,
            at_most: 3,
            password: "cdefg",
        };
        assert_eq!(policy2.is_valid(), false);
        let policy3 = PasswordPolicy {
            mandated_char: 'c',
            at_least: 2,
            at_most: 9,
            password: "ccccccccc",
        };
        assert_eq!(policy3.is_valid(), true);
    }
}
