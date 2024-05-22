pub mod presenter_mod {
    use crate::{
        const_util::const_util_mod::{
            is_input_from_keyword, is_input_import_keyword, CLOSE_CURLY_BRACKET, IMPORT_KEYWORD,
            OPEN_CURLY_BRACKET,
        },
        error::error_mod::Error,
    };

    use std::collections::HashMap;

    pub struct ParsedPresenter {
        pub markup: String,
        pub imports: HashMap<String, String>,
    }

    /// Given a line of chars which is assumed to be an import statement, updates the `imports` map
    /// if import format is correct and returns `Ok`; else, returns the `Err` variant with an explanation.
    fn read_imports(line: String, imports: &mut HashMap<String, String>) -> Result<(), Error> {
        let original_line = line.clone();
        let line_parts: Vec<String> = line.split_whitespace().map(|x| x.to_owned()).collect();
        if line_parts.len() != 4
            || !is_input_import_keyword(&line_parts[0])
            || !is_input_from_keyword(&line_parts[2])
        {
            let error_msg = format!("Import format is wrong; make sure your import statement matches the following format: `import Component from \"/path/to/component\"`. You provided: {original_line}.");
            return Err(Error::ParsingError(error_msg));
        }
        let component = line_parts[1].to_owned();
        let mut path = line_parts[3].to_owned();
        if component.starts_with(OPEN_CURLY_BRACKET) || component.ends_with(CLOSE_CURLY_BRACKET) {
            let error_msg = "Import format is wrong; you are most likely using named imports while currently only default exports are supported.".to_owned();
            return Err(Error::ParsingError(error_msg));
        }
        path = path.replace("\"", "").replace(";", "");
        imports.insert(component, path);
        return Ok(());
    }

    /// Removes any line which contains nothing but whitespace characters.
    fn trim_markup(markup: Vec<String>) -> String {
        let mut res = String::from("");
        for line in markup {
            if line.trim() != "" {
                res.push_str(&line);
            }
        }
        res
    }

    /// Given a string, parses its content into a `ParsedPresenter`, if done successfully; else,
    /// returns a `Err` variant which contains the reason why.
    pub fn parse_presenter(presenter: &String) -> Result<ParsedPresenter, Error> {
        let split_presenter: Vec<&str> = presenter.trim().split('\n').collect();
        let mut in_markup = false;
        let mut imports: HashMap<String, String> = HashMap::new();
        let mut markup: Vec<String> = Vec::new();
        for line in split_presenter {
            if line.trim().starts_with(IMPORT_KEYWORD) && !in_markup {
                let read_import_result = read_imports(line.to_owned(), &mut imports);
                if let Result::Err(err) = read_import_result {
                    return Err(err);
                }
            } else {
                if !in_markup {
                    in_markup = true;
                }
                markup.push(line.to_owned());
            }
        }
        let markup = trim_markup(markup);

        Ok(ParsedPresenter { imports, markup })
    }

    #[cfg(test)]
    mod tests {
        use regex::Regex;

        use super::*;

        #[test]
        fn test_read_imports_invalid_format_1() {
            let line = String::from("from x import z");
            let mut imports: HashMap<String, String> = HashMap::new();
            let read_imports_result = read_imports(line, &mut imports);
            assert!(
                matches!(read_imports_result, Err(err) if matches!(err, Error::ParsingError(_)))
            );
        }

        #[test]
        /// `read_imports` should return error when encounters a wrong format of importing.
        fn test_read_imports_invalid_format_2() {
            let line = String::from("improt Hello form \"/some/path\"");
            let mut imports: HashMap<String, String> = HashMap::new();
            let read_imports_result = read_imports(line, &mut imports);
            assert!(
                matches!(read_imports_result, Err(err) if matches!(err, Error::ParsingError(_)))
            );
        }

        #[test]
        /// `read_imports` must return error when import statement uses named imports.
        fn test_read_imports_named_imports() {
            let line = String::from("import {Hello} from \"/some/path\"");
            let mut imports: HashMap<String, String> = HashMap::new();
            let read_imports_result = read_imports(line, &mut imports);
            assert!(
                matches!(read_imports_result, Err(err) if matches!(err, Error::ParsingError(_)))
            );
        }

        #[test]
        /// Given a correct import statement, `read_imports` should update the `imports` mutable
        /// reference with a key of the name of the component and a value of the path to the component.
        fn test_read_imports() {
            let line = String::from("import Hello from \"/some/path\";");
            let mut imports: HashMap<String, String> = HashMap::new();
            let read_imports_result = read_imports(line, &mut imports);
            assert!(
                matches!(read_imports_result, Ok(_))
                    && imports.len() == 1
                    && imports["Hello"] == "/some/path"
            );
        }

        #[test]
        /// `parse_presenter` must return splitted markup and imports if presenter format is
        /// correct.
        fn test_parse_presenter() {
            let pre_markup = "
            <div>
                <div id={state.x[0].name + \"_\" + state.x[2].name}>
                    <div>hi!</div>
                    <ByeWorld />
                </div>
                <Hello />
            </div>";
            let mut presenter = String::from(
                "
            import ByeWorld from \"/test/ByeWorld/ByeWorld.js\";
            import Hello from \"/test/Hello/Hello.js\";
            ",
            );
            presenter.push_str(pre_markup);

            let parsed_presenter_result = parse_presenter(&presenter);
            if parsed_presenter_result.is_ok() {
                let ParsedPresenter { markup, imports } = parsed_presenter_result.unwrap();
                let re = Regex::new(r#"(\s)|(\n)"#).unwrap();
                let actual = re.replace_all(&markup, "").to_string();
                let expected = re.replace_all(&pre_markup, "").to_string();
                assert!(
                    imports.len() == 2
                        && imports["ByeWorld"] == "/test/ByeWorld/ByeWorld.js"
                        && imports["Hello"] == "/test/Hello/Hello.js"
                        && actual == expected
                )
            } else {
                assert!(false);
            }
        }
    }
}
