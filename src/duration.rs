pub mod parser {
    use once_cell::sync::Lazy;
    use regex::{Match, Regex};

    static DURATION_RE_STR: &str = r#"(?i)(?:(?P<h>\d+)h)? ?(?:(?P<m>\d+)m)? ?(?:(?P<s>\d+)s)?"#;
    static DURATION_RE: Lazy<Regex> = Lazy::new(|| Regex::new(DURATION_RE_STR).unwrap());

    pub fn parse_duration(input: &str) -> Result<u64, ()> {
        match DURATION_RE.captures(input.trim()) {
            Some(caps) => {
                let to_s = |x| move |m: Match| m.as_str().parse::<u64>().unwrap() * x;
                let h = caps.name("h").map(to_s(60 * 60));
                let m = caps.name("m").map(to_s(60));
                let s = caps.name("s").map(to_s(1));
                let parts = &[h, m, s];

                if parts.iter().all(Option::is_none) {
                    Err(())
                } else {
                    Ok(parts.iter().fold(0, |a, b| match b {
                        Some(s) => a + s,
                        None => a,
                    }))
                }
            }
            None => Err(()),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::parse_duration;
        use test_case::test_case;

        #[test_case("2h 2m 30s", Ok(7350))]
        #[test_case("2m 30s", Ok(150))]
        #[test_case("2h2m30s", Ok(7350))]
        #[test_case("2m30s", Ok(150))]
        #[test_case("30s", Ok(30))]
        #[test_case("2m", Ok(120))]
        #[test_case("1", Err(()))]
        #[test_case("m", Err(()))]
        fn test_parse_duration(input: &str, expect: Result<u64, ()>) {
            assert_eq!(parse_duration(input), expect)
        }
    }
}
