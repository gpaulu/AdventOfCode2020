use std::{fs::read_to_string, str::FromStr};

mod passport;

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
