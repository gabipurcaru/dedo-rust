use crate::ast::*;
use crate::defaults::ENVIRONMENT;
use crate::runtime::evaluate;
use crate::types::*;
use peg;
use regex::Regex;

peg::parser!(grammar dedo_parser() for str {
    rule _() = quiet!{[' ' | '\t']*}

    rule number() -> f64
        = precedence! {
            n:$("." ['0'..='9']+) { n.parse().unwrap() }
            n:$(['0'..='9']+ "." ['0'..='9']*) { n.parse().unwrap() }

            --

            n:$(['0'..='9']+) { n.parse().unwrap() }
        }

    rule ident() -> String = s:$(['a'..='z' | 'A'..='Z' | '£' | '$' | '€' | '_']+) { s.parse().unwrap() }

    rule term() -> Term = precedence!{
        x:(@) _ "+" _ y:@ { Term::Binary(Box::new(x), Op::Add, Box::new(y)) }
        x:(@) _ "-" _ y:@ { Term::Binary(Box::new(x), Op::Sub, Box::new(y)) }
        "-" _ y:@ { Term::Negate(Box::new(y)) }

        --

        x:(@) _ "*" _ y:@ { Term::Binary(Box::new(x), Op::Mul, Box::new(y)) }
        x:(@) _ "/" _ y:@ { Term::Binary(Box::new(x), Op::Div, Box::new(y)) }

        --

        n:number() _ i:ident() _ "^" t:term() { 
            Term::Binary(
                Box::new(Term::Num(n)), 
                Op::Mul, 
                Box::new(Term::Binary(
                    Box::new(Term::Ident(i)), 
                    Op::Pow, Box::new(t))
                )
            ) 
        }
        n:number() _ i:ident() _ "**" t:term() { 
            Term::Binary(
                Box::new(Term::Num(n)), 
                Op::Mul, 
                Box::new(Term::Binary(
                    Box::new(Term::Ident(i)), 
                    Op::Pow, Box::new(t))
                )
            ) 
        }

        --

        x:@ _ "^" _ y:(@) { Term::Binary(Box::new(x), Op::Pow, Box::new(y)) }
        x:@ _ "**" _ y:(@) { Term::Binary(Box::new(x), Op::Pow, Box::new(y)) }

        --

        n:number() _ i:ident() { Term::Binary(Box::new(Term::Num(n)), Op::Mul, Box::new(Term::Ident(i))) }
        i:ident() _ n:number() { Term::Binary(Box::new(Term::Num(n)), Op::Mul, Box::new(Term::Ident(i))) }
        i:ident() { Term::Ident(i) }

        --

        n:number() { Term::Num(n) }
        "(" _ e:term() _ ")" { e }
    }

    pub rule statement() -> Statement = precedence!{
        _ i:ident() _ "=" _ t:term() _ {  Statement::Assign(i, t) }
        _ from:term() _ "to" _ to:term() _ { Statement::Transform(from, to) }
        _ from:term() _ "in" _ to:term() _ { Statement::Transform(from, to) }

        --

        _ t:term() _ { Statement::Basic(t) }
    }
});

pub fn parse_single<'input>(env: &'input mut Environment, input: &'input str) -> Result<Value, ()> {
    match dedo_parser::statement(input) {
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
