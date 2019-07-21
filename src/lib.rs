#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate lalrpop_util;

#[macro_use]
pub mod types;
pub mod defaults;
pub mod parser;

// test
mod test_parser;
mod test_types;

lalrpop_mod!(pub language);
