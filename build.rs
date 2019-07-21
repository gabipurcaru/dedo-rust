extern crate lalrpop;

use std::fs;
use std::env;
use std::io::Write;
use std::path::Path;

fn split_spec(input: String) -> (String, String) {
    let splits: Vec<(&str, &str)> = input
        .split("\n")
        .map(|line| line.split("|").collect())
        .map(|line: Vec<&str>| (line[0], line[1]))
        .collect();
    let mut left = String::new();
    let mut right = String::new();
    for (left_line, right_line) in splits.iter() {
        left.push_str(left_line.trim());
        right.push_str(right_line.trim());
    }
    (left, right)
}

fn main() {
    lalrpop::process_root().unwrap();

    let paths = fs::read_dir("./spec").unwrap();


    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("spec_tests.rs");
    let mut f = fs::File::create(&dest_path).unwrap();

    for path in paths {
        let filepath = path.unwrap().path();
        let filename = filepath.file_stem().unwrap().to_str().unwrap();
        let contents = fs::read_to_string(filepath.clone()).unwrap();

        let (left, right) = split_spec(contents);

        f.write_all(format!("
            #[test]
            pub fn spec_{}() {{
                assert_eq!(
                    language::TermParser::new().parse(&mut ENVIRONMENT.clone(), \"{}\"),
                    language::TermParser::new().parse(&mut ENVIRONMENT.clone(), \"{}\"),
                );
            }}
        ", filename, left, right).as_bytes()).unwrap();
    }
}