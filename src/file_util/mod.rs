pub mod file_util_mod {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub struct ParsedFile {
        pub markup: String,
        pub imports: HashMap<String, String>,
    }

    fn read_imports(line: String, imports: &mut HashMap<String, String>) {
        let original_line = line.clone();
        let line = line.replace("import", "").replace("from", "");
        let line_parts: Vec<String> = line.split_whitespace().map(|x| x.to_owned()).collect();
        if line_parts.len() != 2 {
            panic!(
                "import format is wrong; make sure you are using default imports like this: `import Component from \"/path/to/component\"`. You provided: {original_line}."
            )
        }
        imports.insert(line_parts[0].to_owned(), line_parts[1].to_owned());
    }

    fn trim_markup(markup: Vec<String>) -> String {
        let mut res = String::from("");
        for line in markup {
            if line.trim() != "" {
                res.push_str(&line);
            }
        }
        res
    }

    pub fn read_file(path: &String) -> ParsedFile {
        let file = File::open(path);
        if let Result::Err(err) = file {
            panic!("There was an error reading the file specified at {path} : {err}")
        }
        let file = file.unwrap();
        let reader = BufReader::new(file);
        let mut markup: Vec<String> = Vec::new();
        let mut imports: HashMap<String, String> = HashMap::new();
        let mut in_markup = false;
        for line in reader.lines() {
            if let Result::Err(err) = line {
                panic!("There was an error reading content of the file specified at {path} : {err}")
            }
            let line = line.unwrap();
            if line.starts_with("import") && !in_markup {
                read_imports(line, &mut imports);
            } else {
                if !in_markup {
                    in_markup = true;
                }
                markup.push(line);
            }
        }
        let markup = trim_markup(markup);

        ParsedFile { imports, markup }
    }
}
