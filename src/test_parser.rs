#[cfg(test)]
mod tests {
    use super::super::defaults::ENVIRONMENT;
    use super::super::types::*;

    lalrpop_mod!(language);

    #[test]
    fn basic_parse() {
        assert_eq!(
            language::TermParser::new().parse(&mut ENVIRONMENT.clone(), "$1234"),
            Ok(Value::simple(1234.0, "$")),
        );

        assert_eq!(
            language::TermParser::new().parse(&mut ENVIRONMENT.clone(), "1234 usd"),
            Ok(Value::simple(1234.0, "usd")),
        );
    }

    #[test]
    fn parse_negation() {
        assert_eq!(
            language::TermParser::new().parse(&mut ENVIRONMENT.clone(), "-1234   usd"),
            Ok(Value::simple(-1234.0, "usd")),
        );
    }

    #[test]
    fn parse_addition() {
        assert_eq!(
            language::TermParser::new().parse(&mut ENVIRONMENT.clone(), "123 usd + 12 usd"),
            Ok(Value::simple(135.0, "usd")),
        );

        assert_eq!(
            language::TermParser::new().parse(&mut ENVIRONMENT.clone(), "123 usd - 12 usd"),
            Ok(Value::simple(111.0, "usd")),
        );

        assert_eq!(
            language::TermParser::new().parse(&mut ENVIRONMENT.clone(), "-123 usd - 12 usd * 14"),
            Ok(Value::simple(-291.0, "usd")),
        );

        assert_eq!(
            language::TermParser::new().parse(&mut ENVIRONMENT.clone(), "2 * 3 + 4"),
            Ok(Value::unitless(10.0)),
        );

        assert_eq!(
            language::TermParser::new().parse(&mut ENVIRONMENT.clone(), "2 * (3 + 4)"),
            Ok(Value::unitless(14.0)),
        );
    }

    #[test]
    fn parse_power() {
        assert_eq!(
            language::TermParser::new().parse(&mut ENVIRONMENT.clone(), "2 ** 3"),
            Ok(Value::unitless(8.0)),
        );
        assert_eq!(
            language::TermParser::new().parse(&mut ENVIRONMENT.clone(), "($2) ** 3"),
            Ok(Value::new(8.0, units!("$" to 3))),
        );

        assert_eq!(
            language::TermParser::new().parse(&mut ENVIRONMENT.clone(), "($1 + $1) ** (1 + 4 - 2)"),
            Ok(Value::new(8.0, units!("$" to 3))),
        );
    }

    // these are the tests generated at build time from the ./spec folder
    include!(concat!(env!("OUT_DIR"), "/spec_tests.rs"));
}
