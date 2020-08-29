#![feature(test)]
#![feature(box_patterns)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
pub mod types;
pub mod defaults;
pub mod parser;
pub mod ast;
pub mod runtime;

// test
mod test_parser;
mod test_types;
