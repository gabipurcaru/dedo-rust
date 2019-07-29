use crate::defaults::ENVIRONMENT;
use crate::types::*;

lalrpop_mod!(language);

pub fn parse_single<'input>(env: &mut Environment, input: &'input str) -> Result<Value, ()> {
    match language::TermParser::new().parse(env, input) {
        Ok(Ok(v)) => Ok(v),
        _ => Err(()),
    }
}

pub fn parse<'input>(input: &'input str) -> Vec<Result<Value, ()>> {
    // parse the input line by line
    let lines: Vec<&str> = input.split("\n").collect();
    let mut env = ENVIRONMENT.clone();
    let mut res = Vec::new();
    for line in lines.iter() {
        let val: Result<Value, ()> = parse_single(&mut env, line);
        res.push(val.clone());
        env.add_entry(val);
    }

    res
}
