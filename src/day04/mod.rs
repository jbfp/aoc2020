static FILE: &str = include_str!("../inputs/4.txt");

lazy_static! {
    static ref PASSPORTS: Vec<&'static str> = FILE.split("\n\n").collect();
}

pub(crate) fn part1() {
    println!("day 4 part 1");

    let num_valid_passports = PASSPORTS.iter().filter(contains_all_keys).count();

    println!("{:#?} valid passports", num_valid_passports);
}

fn contains_all_keys(s: &&&str) -> bool {
    s.contains("byr:")
        && s.contains("iyr:")
        && s.contains("eyr:")
        && s.contains("hgt:")
        && s.contains("hcl:")
        && s.contains("ecl:")
        && s.contains("pid:")
}

pub(crate) fn part2() {
    println!("day 4 part 2");

    let num_valid_passports = PASSPORTS
        .iter()
        .filter(contains_all_keys)
        .filter(|s| {
            s.split_ascii_whitespace()
                .all(|field| match field.split_at(4) {
                    ("byr:", byr) => {
                        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
                        (1920..=2002).contains(&byr.parse().unwrap_or_default())
                    }

                    ("iyr:", iyr) => {
                        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
                        (2010..=2020).contains(&iyr.parse().unwrap_or_default())
                    }

                    ("eyr:", eyr) => {
                        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
                        (2020..=2030).contains(&eyr.parse().unwrap_or_default())
                    }

                    ("hgt:", hgt) => {
                        // hgt (Height) - a number followed by either cm or in:
                        // - If cm, the number must be at least 150 and at most 193.
                        // - If in, the number must be at least 59 and at most 76.
                        if let Some(rest) = hgt.strip_suffix("cm") {
                            (150..=193).contains(&rest.parse().unwrap_or_default())
                        } else if let Some(rest) = hgt.strip_suffix("in") {
                            (59..=76).contains(&rest.parse().unwrap_or_default())
                        } else {
                            false
                        }
                    }

                    ("hcl:", hcl) => {
                        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
                        match hcl.strip_prefix('#') {
                            Some(rest) => {
                                rest.len() == 6 && rest.chars().all(|c| char::is_digit(c, 16))
                            }
                            None => false,
                        }
                    }

                    ("ecl:", ecl) => {
                        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
                        static EYE_COLORS: [&str; 7] =
                            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                        EYE_COLORS.contains(&ecl)
                    }

                    ("pid:", pid) => {
                        // pid (Passport ID) - a nine-digit number, including leading zeroes.
                        pid.len() == 9 && pid.chars().all(|c| char::is_digit(c, 10))
                    }

                    ("cid:", _cid) => {
                        // cid (Country ID) - ignored, missing or not.
                        true
                    }
                    _ => false,
                })
        })
        .count();

    println!("{:#?} truly valid passports", num_valid_passports);
}
