#[macro_use]
extern crate lazy_static;

#[macro_use]
pub mod types;
pub mod defaults;

// test
mod test_types;

use defaults::{ENVIRONMENT};

fn main() {
    println!("Hello, world - {:?}", *ENVIRONMENT);
}
