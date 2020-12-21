use crate::common::read_input;
use anyhow::{bail, Result};
use regex::Regex;

macro_rules! make_regex {
    ($x:ident) => {
        lazy_static::lazy_static! {
            pub static ref $x: regex::Regex =
                Regex::new(&format!(r"{}:([^ ]+)", stringify!($x))).unwrap();
        }
    };
}

lazy_static::lazy_static! {
    pub static ref HEIGHT_CM: regex::Regex =
        Regex::new(r"^([0-9]+)cm\z").unwrap();
}
lazy_static::lazy_static! {
    pub static ref HEIGHT_INCH: regex::Regex =
        Regex::new(r"^([0-9]+)in\z").unwrap();
}
lazy_static::lazy_static! {
    pub static ref HAIR_CLR: regex::Regex =
        Regex::new(r"^#[0-9|a-f]{6}\z").unwrap();
}
lazy_static::lazy_static! {
    pub static ref EYE_CLR: regex::Regex =
        Regex::new(r"^(?:amb|blu|brn|gry|grn|hzl|oth)\z").unwrap();
}
lazy_static::lazy_static! {
    pub static ref PID_NO: regex::Regex =
        Regex::new(r"^\d{9}\z").unwrap();
}

make_regex!(byr);
make_regex!(iyr);
make_regex!(eyr);
make_regex!(hgt);
make_regex!(hcl);
make_regex!(ecl);
make_regex!(pid);
make_regex!(cid);

#[derive(Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl Passport {
    fn parse_from_string(input: &String) -> Result<Passport> {
        Ok(Passport {
            byr: byr.find(input).ok_or(anyhow::anyhow!(""))?.as_str().into(),
            iyr: iyr.find(input).ok_or(anyhow::anyhow!(""))?.as_str().into(),
            eyr: eyr.find(input).ok_or(anyhow::anyhow!(""))?.as_str().into(),
            hgt: hgt.find(input).ok_or(anyhow::anyhow!(""))?.as_str().into(),
            hcl: hcl.find(input).ok_or(anyhow::anyhow!(""))?.as_str().into(),
            ecl: ecl.find(input).ok_or(anyhow::anyhow!(""))?.as_str().into(),
            pid: pid.find(input).ok_or(anyhow::anyhow!(""))?.as_str().into(),
            cid: cid.find(input).map(|x| x.as_str().into()),
        })
    }

    fn verify(&self) -> Result<()> {
        verify_byr(&self.byr)?;
        verify_iyr(&self.iyr)?;
        verify_eyr(&self.eyr)?;
        verify_hgt(&self.hgt)?;
        verify_hcl(&self.hcl)?;
        verify_ecl(&self.ecl)?;
        verify_pid(&self.pid)?;

        Ok(())
    }
}

fn verify_byr(input: &str) -> Result<()> {
    // Get capture
    let inner = byr.captures(input).unwrap().get(1).unwrap();

    let year = inner.as_str().parse::<u32>()?;
    if 1920 <= year && year <= 2002 {
        return Ok(());
    } else {
        bail!("byr");
    }
}

fn verify_iyr(input: &str) -> Result<()> {
    // Get capture
    let inner = iyr.captures(input).unwrap().get(1).unwrap();

    let year = inner.as_str().parse::<u32>()?;
    if 2010 <= year && year <= 2020 {
        return Ok(());
    } else {
        bail!("iyr");
    }
}

fn verify_eyr(input: &str) -> Result<()> {
    // Get capture
    let inner = eyr.captures(input).unwrap().get(1).unwrap();

    let year = inner.as_str().parse::<u32>()?;
    if 2020 <= year && year <= 2030 {
        return Ok(());
    } else {
        bail!("eyr");
    }
}

fn verify_hgt(input: &str) -> Result<()> {
    // Get capture
    let inner = hgt.captures(input).unwrap().get(1).unwrap().as_str();

    // First check it matches one of HEIGHT_CM or HEIGHT_INCH
    if !HEIGHT_CM.is_match(inner) && !HEIGHT_INCH.is_match(inner) {
        bail!("hgt");
    }

    if let Some(capture) = HEIGHT_CM.captures(inner) {
        let height = capture.get(1).unwrap().as_str().parse::<u32>()?;
        if 150 <= height && height <= 193 {
            return Ok(());
        } else {
            bail!("hgt_cm");
        }
    }

    if let Some(capture) = HEIGHT_INCH.captures(inner) {
        let height = capture.get(1).unwrap().as_str().parse::<u32>()?;
        if 59 <= height && height <= 76 {
            return Ok(());
        } else {
            bail!("hgt_in");
        }
    }

    bail!("hgt");
}

fn verify_hcl(input: &str) -> Result<()> {
    // Get capture
    let inner = hcl.captures(input).unwrap().get(1).unwrap().as_str();

    if let true = HAIR_CLR.is_match(inner) {
        return Ok(());
    } else {
        bail!("hcl")
    }
}

fn verify_ecl(input: &str) -> Result<()> {
    // Get capture
    let inner = ecl.captures(input).unwrap().get(1).unwrap().as_str();

    if EYE_CLR.is_match(inner) {
        return Ok(());
    } else {
        bail!("ecl")
    }
}

fn verify_pid(input: &str) -> Result<()> {
    // Get capture
    let inner = pid.captures(input).unwrap().get(1).unwrap().as_str();

    if let true = PID_NO.is_match(inner) {
        return Ok(());
    } else {
        bail!("pid")
    }
}

pub fn day_four() -> Result<()> {
    let lines = read_input("input/day_four.txt")?;

    // Strategy: Read. Concat until you hit an enpty line. Turn that into a passport entry.
    let lines: Vec<_> = lines.collect();

    let mut passports = vec![];
    let mut current_passport = String::new();
    for line in lines {
        if line == String::new() {
            passports.push(current_passport.clone());
            current_passport = String::new();
        } else {
            current_passport.push_str(" ");
            current_passport.push_str(line.as_str())
        }
    }
    passports.push(current_passport);

    part_two(passports);
    Ok(())
}

fn part_one(passports: Vec<String>) {
    let valid_passports = passports
        .iter()
        .map(|passport| Passport::parse_from_string(passport))
        .filter(|passport| passport.is_ok())
        .count();

    println!("{:?}", valid_passports);
}

fn part_two(passports: Vec<String>) {
    let valid_passports = passports
        .iter()
        .map(|passport| Passport::parse_from_string(passport))
        .filter(|passport| passport.is_ok())
        .map(|passport| passport.unwrap())
        .filter(|passport| passport.verify().is_ok())
        .count();

    println!("{:?}", valid_passports);
}

#[test]
fn test_parse_passport() {
    let passport: String =
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm".into();

    assert!(byr.find(&passport).is_some());
}

#[test]
fn test_verify_byr() {
    let byr_valid = "byr:2002";
    let byr_invalid = "byr:2003";

    assert!(verify_byr(byr_valid).is_ok());
    assert!(verify_byr(byr_invalid).is_err());
}

#[test]
fn test_verify_hgt() {
    let hgt_valid_in = "hgt:60in";
    let hgt_valid_cm = "hgt:190cm";
    let hgt_invalid_in = "hgt:190in";
    let hgt_invalid_cm = "hgt:190";

    assert!(verify_hgt(hgt_valid_in).is_ok());
    assert!(verify_hgt(hgt_valid_cm).is_ok());
    assert!(verify_hgt(hgt_invalid_in).is_err());
    assert!(verify_hgt(hgt_invalid_cm).is_err());
}

#[test]
fn test_verify_hcl() {
    let hcl_valid = "hcl:#123abc";
    let hcl_invalid_letter = "hcl:#123abz";
    let hcl_invalid_hash = "hcl:123abc";

    assert!(verify_hcl(hcl_valid).is_ok());
    assert!(verify_hcl(hcl_invalid_letter).is_err());
    assert!(verify_hcl(hcl_invalid_hash).is_err());
}

#[test]
fn test_verify_ecl() {
    let ecl_valid = "ecl:brn";
    let ecl_invalid = "ecl:wat";

    assert!(verify_ecl(ecl_valid).is_ok());
    assert!(verify_ecl(ecl_invalid).is_err());
}

#[test]
fn test_verify_pid() {
    let pid_valid = "pid:000000001";
    let pid_invalid = "pid:01234567896";

    assert!(verify_pid(pid_valid).is_ok());
    assert!(verify_pid(pid_invalid).is_err());
}
