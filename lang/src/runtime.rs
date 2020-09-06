use super::ast::*;
use super::types::*;

pub fn evaluate<'a>(env: &'a mut Environment, statement: Statement) -> Result<Value, ()> {
    match statement {
        Statement::Assign(ident, term) => {
            let t = evaluate_term(env, term);
            env.assign(ident, t?)
        },
        Statement::Transform(from, to) => {
            let left = evaluate_term(env, from);
            let right = evaluate_term(env, to);
            Ok(env.convert(
                left?,
                right?,
            ))
        },
        Statement::Basic(term) => evaluate_term(env, term),
    }
}

pub fn evaluate_term<'a>(env: &'a mut Environment, term: Term) -> Result<Value, ()> {
    match term {
        Term::Binary(box left, op, box right) => {
            let left_value = evaluate_term(env, left);
            let right_value = evaluate_term(env, right);

            match op {
                Op::Add => Ok(env.add(left_value?, right_value?)),
                Op::Sub => Ok(env.sub(left_value?, right_value?)),
                Op::Mul => Ok(env.mul(left_value?, right_value?)),
                Op::Div => Ok(env.div(left_value?, right_value?)),
                Op::Pow => Ok(env.pow(left_value?, right_value?)),
            }
        },
        Term::Negate(box term) => {
            let value = evaluate_term(env, term);
            Ok(Value::negate(value?))
        },
        Term::Ident(ident) => env.ident(ident),
        Term::Num(num) => Ok(Value::unitless(num)),
    }
}
