pub mod parser {
    use once_cell::sync::Lazy;
    use regex::{Match, Regex};
    use thiserror::Error;

    static DURATION_RE_STR: &str = r#"(?i)(?:(?P<h>\d+)h)? ?(?:(?P<m>\d+)m)? ?(?:(?P<s>\d+)s)?"#;
    static DURATION_RE: Lazy<Regex> = Lazy::new(|| Regex::new(DURATION_RE_STR).unwrap());

    #[derive(Error, Debug, PartialEq, Clone, Copy)]
    #[error("failed to parse string as a duration")]
    pub struct ParseDurationError;

    pub fn parse_duration(input: &str) -> Result<u64, ParseDurationError> {
        let caps = DURATION_RE.captures(input.trim()).unwrap();
        let to_s = |x| move |m: Match| m.as_str().parse::<u64>().unwrap() * x;
        let h = caps.name("h").map(to_s(60 * 60));
        let m = caps.name("m").map(to_s(60));
        let s = caps.name("s").map(to_s(1));
        let parts = &[h, m, s];

        if parts.iter().all(Option::is_none) {
            Err(ParseDurationError)
        } else {
            Ok(parts.iter().fold(0, |a, b| match b {
                Some(s) => a + s,
                None => a,
            }))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{parse_duration, ParseDurationError};
        use test_case::test_case;

        #[test_case("2h 2m 30s", Ok(7350))]
        #[test_case("2m 30s", Ok(150))]
        #[test_case("2h2m30s", Ok(7350))]
        #[test_case("2m30s", Ok(150))]
        #[test_case("30s", Ok(30))]
        #[test_case("2m", Ok(120))]
        #[test_case("1", Err(ParseDurationError))]
        #[test_case("m", Err(ParseDurationError))]
        #[test_case("", Err(ParseDurationError))]
        fn test_parse_duration(input: &str, expect: Result<u64, ParseDurationError>) {
            assert_eq!(parse_duration(input), expect)
        }
    }
}
