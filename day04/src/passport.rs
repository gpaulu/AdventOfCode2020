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
    pub issue_year: i32,
    pub expiration_year: i32,
    pub height: Height,
    pub hair_color: String,
    pub eye_color: EyeColor,
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

//probably could have saved some headache in debugging by not cutting this corner
//If I'm not going to do proper error handling, it is probably better to just `unwrap`
//  rather than a unit error that throws away all context
//Or use Box<Error>
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
            issue_year: validate_iyr(self.issue_year)?,
            expiration_year: validate_eyr(self.expiration_year)?,
            height: validate_hgt(self.height)?,
            hair_color: validate_hcl(self.hair_color)?,
            eye_color: validate_ecl(self.eye_color)?,
            passport_id: validate_pid(self.passport_id)?,
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
    let i: i32 = issue_year
        .and_then(|s| s.parse().ok())
        .ok_or(PassportError {})?;
    if i < 2010 || i > 2020 {
        return Err(PassportError {});
    }
    Ok(i)
}
fn validate_eyr(expiration_year: Option<String>) -> Result<i32> {
    let i: i32 = expiration_year
        .and_then(|s| s.parse().ok())
        .ok_or(PassportError {})?;
    if i < 2020 || i > 2030 {
        return Err(PassportError {});
    }
    Ok(i)
}
fn validate_hgt(height: Option<String>) -> Result<Height> {
    //not really happy with this
    let height = height.ok_or(PassportError {})?;
    let unit = height
        .as_bytes()
        .iter()
        .rev()
        .take(2)
        .rev()
        .cloned()
        .collect::<Vec<_>>();
    let value = height
        .as_bytes()
        .iter()
        .rev()
        .skip(2)
        .rev()
        .cloned()
        .collect::<Vec<_>>();
    let value = std::str::from_utf8(value.as_slice())
        .map_err(|_| PassportError {})?
        .parse::<i32>()
        .map_err(|_| PassportError {})?;
    Ok(match unit.as_slice() {
        b"cm" => {
            if value < 150 || value > 193 {
                return Err(PassportError {});
            } else {
                Height::Cm(value)
            }
        }
        b"in" => {
            if value < 59 || value > 76 {
                return Err(PassportError {});
            } else {
                Height::Inch(value)
            }
        }
        _ => return Err(PassportError {}),
    })
}
fn validate_hcl(hair_color: Option<String>) -> Result<String> {
    let hair_color = hair_color.ok_or(PassportError {})?;
    if hair_color.len() != 7 {
        return Err(PassportError {});
    }
    let iter = hair_color.as_bytes().iter().cloned().enumerate();
    for (i, c) in iter {
        if i == 0 {
            if c != b'#' {
                return Err(PassportError {});
            }
            continue;
        }
        match c {
            b'0'..=b'9' => continue,
            b'a'..=b'f' => continue,
            _ => return Err(PassportError {}),
        }
    }
    Ok(hair_color)
}
fn validate_ecl(eye_color: Option<String>) -> Result<EyeColor> {
    Ok(match eye_color.ok_or(PassportError {})?.as_str() {
        "amb" => EyeColor::Amb,
        "blu" => EyeColor::Blu,
        "brn" => EyeColor::Brn,
        "gry" => EyeColor::Gry,
        "grn" => EyeColor::Grn,
        "hzl" => EyeColor::Hzl,
        "oth" => EyeColor::Oth,
        _ => return Err(PassportError {}),
    })
}
fn validate_pid(passport_id: Option<String>) -> Result<String> {
    let passport_id = passport_id.ok_or(PassportError {})?;
    if passport_id.len() != 9 {
        return Err(PassportError {});
    }
    let iter = passport_id.as_bytes().iter().cloned();
    for c in iter {
        match c {
            b'0'..=b'9' => continue,
            _ => return Err(PassportError {}),
        }
    }
    Ok(passport_id)
}

impl FromStr for PassportBuilder {
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
        Ok(builder)
    }
}

impl FromStr for Passport {
    type Err = PassportError;

    fn from_str(s: &str) -> Result<Self> {
        PassportBuilder::from_str(s)?.build()
    }
}

impl FromStr for ValidatedPassport {
    type Err = PassportError;

    fn from_str(s: &str) -> Result<Self> {
        PassportBuilder::from_str(s)?.build_validated()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passport_from_str() {
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
        assert_eq!(validate_iyr(Some("2011".into())).unwrap(), 2011);
        assert_eq!(validate_iyr(Some("2010".into())).unwrap(), 2010);
        assert_eq!(validate_iyr(Some("2020".into())).unwrap(), 2020);
        assert_eq!(
            validate_iyr(Some("2009".into())).unwrap_err(),
            PassportError {}
        );
        assert_eq!(
            validate_iyr(Some("2021".into())).unwrap_err(),
            PassportError {}
        );
    }
    #[test]
    fn validated_eyr() {
        assert_eq!(validate_eyr(Some("2021".into())).unwrap(), 2021);
        assert_eq!(validate_eyr(Some("2020".into())).unwrap(), 2020);
        assert_eq!(validate_eyr(Some("2030".into())).unwrap(), 2030);
        assert_eq!(
            validate_eyr(Some("2019".into())).unwrap_err(),
            PassportError {}
        );
        assert_eq!(
            validate_eyr(Some("2031".into())).unwrap_err(),
            PassportError {}
        );
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
    #[test]
    fn validated_pid() {
        assert_eq!(validate_pid(Some("000000001".into())).unwrap(), "000000001");
        assert_eq!(
            validate_hcl(Some("0123456789".into())).unwrap_err(),
            PassportError {}
        );
    }
    #[test]
    fn passports_invalid() {
        let pstr = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926";
        assert_eq!(
            PassportError {},
            ValidatedPassport::from_str(pstr).unwrap_err()
        );
        let pstr = "iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946";
        assert_eq!(
            PassportError {},
            ValidatedPassport::from_str(pstr).unwrap_err()
        );
        let pstr = "hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277";
        assert_eq!(
            PassportError {},
            ValidatedPassport::from_str(pstr).unwrap_err()
        );
        let pstr = "hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        assert_eq!(
            PassportError {},
            ValidatedPassport::from_str(pstr).unwrap_err()
        );
    }
    #[test]
    fn passports_valid() {
        let pstr = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f";
        assert_eq!(
            ValidatedPassport {
                birth_year: 1980,
                issue_year: 2012,
                expiration_year: 2030,
                height: Height::Inch(74),
                hair_color: "#623a2f".into(),
                eye_color: EyeColor::Grn,
                passport_id: "087499704".into(),
                country_id: None
            },
            ValidatedPassport::from_str(pstr).unwrap()
        );
        let pstr = "eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm";
        assert_eq!(
            ValidatedPassport {
                birth_year: 1989,
                issue_year: 2014,
                expiration_year: 2029,
                height: Height::Cm(165),
                hair_color: "#a97842".into(),
                eye_color: EyeColor::Blu,
                passport_id: "896056539".into(),
                country_id: Some("129".into())
            },
            ValidatedPassport::from_str(pstr).unwrap()
        );
        let pstr = "hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022";
        assert_eq!(
            ValidatedPassport {
                birth_year: 2001,
                issue_year: 2015,
                expiration_year: 2022,
                height: Height::Cm(164),
                hair_color: "#888785".into(),
                eye_color: EyeColor::Hzl,
                passport_id: "545766238".into(),
                country_id: Some("88".into())
            },
            ValidatedPassport::from_str(pstr).unwrap()
        );
        let pstr = "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert_eq!(
            ValidatedPassport {
                birth_year: 1944,
                issue_year: 2010,
                expiration_year: 2021,
                height: Height::Cm(158),
                hair_color: "#b6652a".into(),
                eye_color: EyeColor::Blu,
                passport_id: "093154719".into(),
                country_id: None
            },
            ValidatedPassport::from_str(pstr).unwrap()
        );
    }
}
