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
    println!("height is ok");
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
}
