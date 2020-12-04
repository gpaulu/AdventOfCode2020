use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

fn count_valid_passports(passports: &str) -> usize {
    todo!();
}

mod passport {
    use std::str::FromStr;

    #[derive(Debug, Eq, PartialEq)]
    pub struct Passport {
        pub birth_year: String,
        pub issue_year: String,
        pub expiration_year: String,
        pub height: String,
        pub hair_color: String,
        pub eye_color: String,
        pub passport_id: String,
        pub country_id: Option<String>,
    }

    #[derive(Debug, Default)]
    struct PassportBuilder {
        birth_year: Option<String>,
        issue_year: Option<String>,
        expiration_year: Option<String>,
        height: Option<String>,
        hair_color: Option<String>,
        eye_color: Option<String>,
        passport_id: Option<String>,
        country_id: Option<String>,
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct PassportError {}

    impl PassportBuilder {
        fn build(self) -> Result<Passport, PassportError> {
            Ok(Passport {
                birth_year: self.birth_year.ok_or(PassportError {})?,
                issue_year: self.issue_year.ok_or(PassportError {})?,
                expiration_year: self.expiration_year.ok_or(PassportError {})?,
                height: self.height.ok_or(PassportError {})?,
                hair_color: self.hair_color.ok_or(PassportError {})?,
                eye_color: self.eye_color.ok_or(PassportError {})?,
                passport_id: self.passport_id.ok_or(PassportError {})?,
                country_id: self.country_id,
            })
        }
    }

    impl FromStr for Passport {
        type Err = PassportError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut builder = PassportBuilder::default();
            for entry in s.split_whitespace() {
                let parts: Vec<_> = entry.split(':').collect();
                match parts[0] {
                    "byr" => builder.birth_year = Some(parts[1].into()),
                    "iyr" => builder.issue_year = Some(parts[1].into()),
                    "eyr" => builder.expiration_year = Some(parts[1].into()),
                    "hgt" => builder.height = Some(parts[1].into()),
                    "hcl" => builder.hair_color = Some(parts[1].into()),
                    "ecl" => builder.eye_color = Some(parts[1].into()),
                    "pid" => builder.passport_id = Some(parts[1].into()),
                    "cid" => builder.country_id = Some(parts[1].into()),
                    _ => return Err(PassportError {}),
                }
            }
            builder.build()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        assert_eq!(count_valid_passports(input), 2);
    }

    #[test]
    fn passport_from_str() {
        use crate::passport::*;

        let pstr1 = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";
        assert_eq!(
            Passport {
                birth_year: "1937".into(),
                issue_year: "2017".into(),
                expiration_year: "2020".into(),
                height: "183cm".into(),
                hair_color: "#fffffd".into(),
                eye_color: "gry".into(),
                passport_id: "860033327".into(),
                country_id: Some("147".into())
            },
            Passport::from_str(pstr1).unwrap()
        );

        let pstr2 = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929";
        assert_eq!(PassportError {}, Passport::from_str(pstr2).unwrap_err());

        let pstr3 = "hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm";
        assert_eq!(
            Passport {
                birth_year: "1931".into(),
                issue_year: "2013".into(),
                expiration_year: "2024".into(),
                height: "179cm".into(),
                hair_color: "#ae17e1".into(),
                eye_color: "brn".into(),
                passport_id: "760753108".into(),
                country_id: None
            },
            Passport::from_str(pstr3).unwrap()
        );

        let pstr4 = "hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        assert_eq!(PassportError {}, Passport::from_str(pstr4).unwrap_err());
    }
}
