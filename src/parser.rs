use crate::defaults::ENVIRONMENT;
use crate::types::*;
use crate::runtime::evaluate;
use regex::Regex;

lalrpop_mod!(language);

pub fn parse_single<'input>(env: &'input mut Environment, input: &'input str) -> Result<Value, ()> {
    match language::TermParser::new().parse(input) {
        Ok(s) => evaluate(env, s),
        _ => Err(()),
    }
}

pub fn parse<'input>(input: &'input str) -> Vec<Result<Value, ()>> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(.*:)?(.*)$").unwrap();
    }
    // parse the input line by line
    let lines: Vec<&str> = input.split("\n").collect();
    let mut env = ENVIRONMENT.clone();
    let mut res = Vec::new();
    for line in lines.iter() {
        let filtered_line = RE.captures(line).unwrap().get(2).unwrap().as_str();
        let val: Result<Value, ()> = parse_single(&mut env, filtered_line);
        res.push(val.clone());
        env.add_entry(val);
    }

    res
}
