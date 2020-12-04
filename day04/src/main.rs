use std::{fs::read_to_string, str::FromStr};

fn main() {
    let input = read_to_string("input.txt").expect("error reading input file");
    println!("{} valid \"passports\" ;)", count_valid_passports(&input));
}

fn count_valid_passports(passports: &str) -> usize {
    passports
        .split("\n\n")
        .filter_map(|s| passport::Passport::from_str(s).ok())
        .count()
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
    pub struct ValidatedPassport {
        pub birth_year: i32,
        pub issue_year: String,
        pub expiration_year: String,
        pub height: String,
        pub hair_color: String,
        pub eye_color: String,
        pub passport_id: String,
        pub country_id: Option<String>,
    }

    #[derive(Debug, Eq, PartialEq)]
    pub enum Height {
        Cm(i32),
        Inch(i32),
    }

    #[derive(Debug, Eq, PartialEq)]
    pub enum EyeColor {
        Amb,
        Blu,
        Brn,
        Gry,
        Grn,
        Hzl,
        Oth,
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct PassportError {}

    type Result<T> = std::result::Result<T, PassportError>;

    impl PassportBuilder {
        fn build(self) -> Result<Passport> {
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

        fn build_validated(self) -> Result<ValidatedPassport> {
            Ok(ValidatedPassport {
                birth_year: validate_byr(self.birth_year)?,
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

    fn validate_byr(birth_year: Option<String>) -> Result<i32> {
        let i: i32 = birth_year
            .and_then(|s| s.parse().ok())
            .ok_or(PassportError {})?;
        if i < 1920 || i > 2002 {
            return Err(PassportError {});
        }
        Ok(i)
    }
    fn validate_iyr(issue_year: Option<String>) -> Result<i32> {
        todo!()
    }
    fn validate_eyr(expiration_year: Option<String>) -> Result<i32> {
        todo!()
    }
    fn validate_hgt(height: Option<String>) -> Result<Height> {
        todo!()
    }
    fn validate_hcl(hair_color: Option<String>) -> Result<String> {
        todo!()
    }
    fn validate_ecl(eye_color: Option<String>) -> Result<EyeColor> {
        todo!()
    }
    fn validate_pid(passport_id: Option<String>) -> Result<String> {
        todo!()
    }

    impl FromStr for Passport {
        type Err = PassportError;

        fn from_str(s: &str) -> Result<Self> {
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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn validated_byr() {
            assert_eq!(validate_byr(Some("2002".into())).unwrap(), 2002);
            assert_eq!(
                validate_byr(Some("2003".into())).unwrap_err(),
                PassportError {}
            );
        }
        #[test]
        fn validated_iyr() {
            todo!()
        }
        #[test]
        fn validated_eyr() {
            todo!()
        }
        #[test]
        fn validated_hgt() {
            assert_eq!(validate_hgt(Some("60in".into())).unwrap(), Height::Inch(60));
            assert_eq!(validate_hgt(Some("190cm".into())).unwrap(), Height::Cm(190));
            assert_eq!(
                validate_hgt(Some("190in".into())).unwrap_err(),
                PassportError {}
            );
            assert_eq!(
                validate_hgt(Some("190".into())).unwrap_err(),
                PassportError {}
            );
        }
        #[test]
        fn validated_hcl() {
            assert_eq!(validate_hcl(Some("#123abc".into())).unwrap(), "#123abc");
            assert_eq!(
                validate_hcl(Some("#123abz".into())).unwrap_err(),
                PassportError {}
            );
            assert_eq!(
                validate_hcl(Some("123abc".into())).unwrap_err(),
                PassportError {}
            );
        }
        #[test]
        fn validated_ecl() {
            assert_eq!(validate_ecl(Some("brn".into())).unwrap(), EyeColor::Brn);
            assert_eq!(
                validate_ecl(Some("wat".into())).unwrap_err(),
                PassportError {}
            );
        }
        fn validated_pid() {
            assert_eq!(validate_pid(Some("000000001".into())).unwrap(), "000000001");
            assert_eq!(
                validate_hcl(Some("0123456789".into())).unwrap_err(),
                PassportError {}
            );
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
