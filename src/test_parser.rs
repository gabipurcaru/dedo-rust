#[cfg(test)]
mod tests {
    use super::super::types::*;

    lalrpop_mod!(language);

    #[test]
    fn basic_parse() {
        assert_eq!(
            language::TermParser::new().parse("$1234"),
            Ok(Value::simple(1234.0, "$")),
        );

        assert_eq!(
            language::TermParser::new().parse("1234 usd"),
            Ok(Value::simple(1234.0, "usd")),
        );
    }

    #[test]
    fn parse_negation() {
        assert_eq!(
            language::TermParser::new().parse("-1234 usd"),
            Ok(Value::simple(-1234.0, "usd")),
        );
    }
}
