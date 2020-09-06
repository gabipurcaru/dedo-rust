use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn split_spec(input: String) -> (String, String) {
    let splits: Vec<(&str, &str)> = input
        .trim()
        .split("\n")
        .map(|line| line.split("|").collect())
        .map(|line: Vec<&str>| (line[0], line[1]))
        .collect();
    let mut left = String::new();
    let mut right = String::new();
    for (left_line, right_line) in splits.iter() {
        left.push_str(left_line.trim());
        left.push_str("\\n");
        right.push_str(right_line.trim());
        right.push_str("\\n");
    }
    (left, right)
}

fn main() {
    let paths = fs::read_dir("./spec").unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("spec_tests.rs");
    let mut output_file = fs::File::create(&dest_path).unwrap();

    for path in paths {
        let filepath = path.unwrap().path();
        let filename = filepath.file_stem().unwrap().to_str().unwrap();
        let contents = fs::read_to_string(filepath.clone()).unwrap();

        if contents.contains("\"") {
            panic!("Input file {} cannot contain double quotes", filename);
        }

        let (left, right) = split_spec(contents);

        output_file
            .write_all(
                format!(
                    "
            #[test]
            pub fn spec_{}() {{
                self::assert_eq!(
                    parse(\"{}\"),
                    parse(\"{}\"),
                );
            }}
        ",
                    filename, right, left
                )
                .as_bytes(),
            )
            .unwrap();
    }
}
