#[cfg(test)]
mod tests {
    lalrpop_mod!(pub language);

    #[test]
    fn basic_parse() {
        assert!(language::TermParser::new().parse("$1234").is_ok());
    }
}
