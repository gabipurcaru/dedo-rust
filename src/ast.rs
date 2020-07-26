pub enum Statement {
    Assign(String, Term),
    Transform(Term, Term),
    Basic(Term),
}

pub enum Term {
    Binary(Box<Term>, Op, Box<Term>),
    Negate(Box<Term>),
    Ident(String),
    Num(f64),
}

pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}