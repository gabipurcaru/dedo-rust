use crate::defaults::ENVIRONMENT;
use crate::types::*;

use lalrpop_util::ParseError;

lalrpop_mod!(language);

pub type ParsingError<'a> = ParseError<usize, language::Token<'a>, &'static str>;

pub fn parse_single<'input>(
    env: &mut Environment,
    input: &'input str,
) -> Result<Value, ParsingError<'input>> {
    language::TermParser::new().parse(env, input)
}

pub fn parse<'input>(input: &'input str) -> Vec<Result<Value, ()>> {
    // parse the input line by line
    let lines: Vec<&str> = input.split("\n").collect();
    let mut env = ENVIRONMENT.clone();
    let mut res = Vec::new();
    for line in lines.iter() {
        let val = language::TermParser::new()
            .parse(&mut env, line)
            .map_err(|_| ());
        res.push(val.clone());
        env.add_entry(val);
    }

    res
}
