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
