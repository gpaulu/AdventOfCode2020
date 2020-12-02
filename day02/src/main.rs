use std::{char::ParseCharError, num::ParseIntError};
use std::{fs::read_to_string, str::FromStr};

struct PasswordPolicy {
    pub mandated_char: char,
    // I feel that min/max names don't specify inclusive range
    pub at_least: i32,
    pub at_most: i32,
    pub password: String,
}

impl PasswordPolicy {
    fn is_valid(&self) -> bool {
        let mandated_char_count = self.password.matches(self.mandated_char).count();
        mandated_char_count >= self.at_least as usize
            && mandated_char_count <= self.at_most as usize
    }
}

#[derive(Debug, derive_more::Display, derive_more::From)]
enum ParsePasswordPolicyError {
    Int(ParseIntError),
    Char(ParseCharError),
}

impl FromStr for PasswordPolicy {
    type Err = ParsePasswordPolicyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        let min_max: Vec<&str> = parts[0].split('-').collect();
        let at_least: i32 = min_max[0].parse()?;
        let at_most: i32 = min_max[1].parse()?;
        let mandated_char: char = parts[1].trim_end_matches(':').parse()?;
        let password = parts[2].to_string();
        Ok(PasswordPolicy {
            mandated_char,
            at_least,
            at_most,
            password,
        })
    }
}

fn main() {
    let input = read_to_string("input.txt").expect("failed to read input file");
    println!("There are {} valid passwords", num_passwords_valid(&input));
}

fn num_passwords_valid(passwords: &str) -> usize {
    passwords
        .lines()
        .filter_map(|s| {
            let valid = s
                .parse::<PasswordPolicy>()
                .expect("policy parse failed")
                .is_valid();
            if valid {
                Some(())
            } else {
                None
            }
        })
        .count()
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
    fn example_policies() {
        let policy1 = PasswordPolicy {
            mandated_char: 'a',
            at_least: 1,
            at_most: 3,
            password: "abcde".into(),
        };
        assert_eq!(policy1.is_valid(), true);
        let policy2 = PasswordPolicy {
            mandated_char: 'b',
            at_least: 1,
            at_most: 3,
            password: "cdefg".into(),
        };
        assert_eq!(policy2.is_valid(), false);
        let policy3 = PasswordPolicy {
            mandated_char: 'c',
            at_least: 2,
            at_most: 9,
            password: "ccccccccc".into(),
        };
        assert_eq!(policy3.is_valid(), true);
    }
    #[test]
    fn policy_too_many() {
        let policy = PasswordPolicy {
            mandated_char: 'a',
            at_least: 1,
            at_most: 3,
            password: "abcdeaaaaa".into(),
        };
        assert_eq!(policy.is_valid(), false);
    }
}
