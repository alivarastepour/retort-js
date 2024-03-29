pub mod tokenizer_mod {
    use std::usize::MIN;

    use serde::de::value;

    pub enum TokenizerState {
        Uninitialized,
        OpenAngleBracket,
        CloseAngleBracket,
        SelfClosingAngleBracket,
        Tag,
        Component,
        Props,
        Text,
        Finalized,
        Unknown,
        Error(String),
    }

    pub enum PropValueWrapperChar {
        OpenCurlyBracket,
        DoubleQuotation,
        SingleQuotation,
        Error(String),
    }

    pub struct CurrentState {
        pub state: TokenizerState,
        pub token: String,
    }

    impl Clone for TokenizerState {
        fn clone(&self) -> Self {
            match self {
                Self::CloseAngleBracket => Self::CloseAngleBracket,
                Self::Uninitialized => Self::Uninitialized,
                Self::OpenAngleBracket => Self::OpenAngleBracket,
                Self::SelfClosingAngleBracket => Self::SelfClosingAngleBracket,
                Self::Props => Self::Props,
                Self::Text => Self::Text,
                Self::Finalized => Self::Finalized,
                Self::Unknown => Self::Unknown,
                Self::Tag => Self::Tag,
                Self::Component => Self::Component,
                Self::Error(err) => Self::Error(err.clone()),
            }
        }
    }

    pub fn proceed_from_uninitialized(markup: &Vec<char>, index: &mut usize) -> CurrentState {
        let max = markup.len();
        loop {
            if *index == max {
                break;
            }
            let current_string = markup[*index].to_string();
            let current = current_string.trim();
            if current == "<" {
                return CurrentState {
                    token: "<".to_owned(),
                    state: TokenizerState::OpenAngleBracket,
                };
            } else if current != "" {
                // TODO: handle error case?
                return CurrentState {
                    token: markup.into_iter().collect(),
                    state: TokenizerState::Finalized,
                };
            }
            *index = *index + 1;
        }
        CurrentState {
            token: "".to_owned(),
            state: TokenizerState::Unknown,
        }
    }

    /// Advances `index` till it reaches the first char that doesn't match with `\s`(any whitespace char).
    /// It will update the `index` mutable reference.
    fn update_starting_tag_index(index: &mut usize, max: usize, markup: &Vec<char>) {
        loop {
            if *index == max {
                break;
            }
            let current_string = markup[*index].to_string();
            let current = current_string.trim();
            if current != "" {
                break;
            }
            *index += 1;
        }
    }

    /// Advances `index` to the end of tag name. It will update mutable references of both `index` and
    /// `tag_name`
    fn update_starting_tag_name(index: &mut usize, tag_name: &mut String, markup: &Vec<char>) {
        loop {
            let mut current = markup[*index].to_string();
            current = current.trim().to_owned();
            if current != "" {
                tag_name.push_str(&current);
                *index += 1;
            } else {
                break;
            }
        }
    }

    fn proceed_from_open_angle_bracket(markup: &Vec<char>, index: &mut usize) -> CurrentState {
        let max = markup.len();
        let mut tag_name = String::from("");

        update_starting_tag_index(index, max, markup);
        if *index == max {
            return CurrentState {
                token: "".to_owned(),
                state: TokenizerState::Error(
                    "No tag name was found after open angle bracket.".to_owned(),
                ),
            };
        }

        update_starting_tag_name(index, &mut tag_name, markup);
        let collected_tag_name: Vec<char> = tag_name.chars().collect();
        let is_valid_string = collected_tag_name.iter().all(|x| x.is_alphabetic());
        if !is_valid_string {
            return CurrentState {
                token: "".to_owned(),
                state: TokenizerState::Error(
                    "Provided tag name '{tag_name}' contains invalid characters.".to_owned(),
                ),
            };
        }
        let first_letter = collected_tag_name[0];
        let is_uppercase = first_letter.is_uppercase();
        if is_uppercase {
            return CurrentState {
                token: tag_name,
                state: TokenizerState::Component,
            };
        } else {
            return CurrentState {
                token: tag_name,
                state: TokenizerState::Tag,
            };
        }
    }

    // fn new_proceed_from_name(markup: &Vec<char>, index: &mut usize){
    //     let max = markup.len();
    //     let
    //     loop {

    //     }
    // }

    fn read_key_of_prop(index: &mut usize, markup: &Vec<char>) -> String {
        let max = markup.len();
        let mut key_value_pair = String::from("");
        loop {
            if *index == max {
                break;
            }
            let mut current = markup[*index].to_string(); // todo: generalize this shit
            current = current.trim().to_owned();
            if current != "" {
                key_value_pair.push_str(&current);
            }
            *index += 1;
            if current == "=" {
                break;
            }
        }
        return key_value_pair;
    }

    fn advance_to_prop_wrapper(index: &mut usize, markup: &Vec<char>) {
        let max = markup.len();
        update_starting_tag_index(index, max, markup);
    }

    fn get_prop_wrapper_char(
        index: &mut usize,
        markup: &Vec<char>,
    ) -> (char, PropValueWrapperChar) {
        let mut current = markup[*index].to_string(); // todo: generalize this shit
        current = current.trim().to_owned();
        *index += 1;
        match current.as_str() {
            "\"" => ('"',PropValueWrapperChar::DoubleQuotation),
            "'" => ('\'',PropValueWrapperChar::SingleQuotation),
            "{" => ('{',PropValueWrapperChar::OpenCurlyBracket),
            _ => (' ',PropValueWrapperChar::Error(
                "`{_}` is not a valid wrapper for value of attributes or props. Wrap values inside `\"\"`, `''` or `{{}}`".to_owned(),
            )),
        }
    }

    fn get_expected_value_end_char(value_start_char: &PropValueWrapperChar) -> String {
        let res;
        match value_start_char {
            PropValueWrapperChar::DoubleQuotation => {
                res = "\"";
            }
            PropValueWrapperChar::Error(_err) => {
                res = "";
            }
            PropValueWrapperChar::OpenCurlyBracket => {
                res = "}";
            }
            PropValueWrapperChar::SingleQuotation => {
                res = "'";
            }
        }

        return res.to_owned();
    }

    fn read_value_of_prop(index: &mut usize, markup: &Vec<char>) -> String {
        advance_to_prop_wrapper(index, markup);
        let (chr, value_wrapper) = get_prop_wrapper_char(index, markup);
        let expected_value_end_char = get_expected_value_end_char(&value_wrapper);
        if let PropValueWrapperChar::Error(err) = value_wrapper {
            // ? what to do
            panic!("wtf"); //todo
        }
        let mut value = String::from(chr);
        let max = markup.len();
        let mut wrapper_stack: Vec<String> = Vec::new();
        loop {
            if *index == max {
                break;
            } // todo : quote marks are no different in the opening or closing variant and that's
              // not handled
            let mut current = markup[*index].to_string(); // todo: generalize this shit
            current = current.trim().to_owned();
            value.push_str(&current);
            if current == expected_value_end_char && wrapper_stack.is_empty() {
                // value.push_str(&current);
                *index += 1;
                break;
            } else if current == expected_value_end_char {
                // value.push_str(&current);
                wrapper_stack.pop();
            }
            *index += 1;
        }
        value
    }

    fn proceed_from_name(markup: &Vec<char>, index: &mut usize) -> CurrentState {
        let max = markup.len();
        update_starting_tag_index(index, max, markup);
        if markup[*index] == '>' {
            CurrentState {
                token: ">".to_owned(),
                state: TokenizerState::CloseAngleBracket,
            }
        } else if markup[*index] == '/' {
            *index += 1; // we want to check if the char after `/` is `>` or not, so we must advance
                         // the index by one; otherwise, call to `update_starting_tag_index`` won't advance the index
                         // because it currently stands at a non-whitespace char(/).
            update_starting_tag_index(index, max, markup);
            let has_closing_angle_bracket = markup[*index] == '>';
            if has_closing_angle_bracket {
                CurrentState {
                    state: TokenizerState::SelfClosingAngleBracket,
                    token: "/>".to_owned(),
                }
            } else {
                CurrentState {
                    state: TokenizerState::Error(
                        "Expected a closing angle bracket, but did not find it.".to_owned(),
                    ),
                    token: "error".to_owned(),
                }
            }
        } else {
            let key = read_key_of_prop(index, markup);
            let value = read_value_of_prop(index, markup);
            let key_value_pair = key + &value;
            // loop {
            //     if *index == max {
            //         break;
            //     }
            //     let mut current = markup[*index].to_string(); // todo: generalize this shit
            //     current = current.trim().to_owned();
            //     if current != "" {
            //         key_value_pair.push_str(&current);
            //     }
            //     *index += 1;
            //     if current == "=" {
            //         break;
            //     }
            // }
            // update_starting_tag_index(index, max, markup);
            // let mut current = markup[*index].to_string(); // todo: generalize this shit
            // current = current.trim().to_owned();
            // *index += 1;
            // let mut value_wrapper = String::from("err");

            // if current == "\"" {
            //     value_wrapper = "dq".to_owned();
            // } else if current == "'" {
            //     value_wrapper = "sq".to_owned();
            // } else if current == "{" {
            //     value_wrapper = "br".to_owned();
            // }
            // loop {
            //     if *index == max {
            //         break;
            //     }
            //     let mut current = markup[*index].to_string(); // todo: generalize this shit
            //     current = current.trim().to_owned();
            // }
            // loop {
            //     if *index == max {
            //         break;
            //     }
            //     let mut current = markup[*index].to_string(); // todo: generalize this shit
            //     current = current.trim().to_owned();
            //     if current == ">" || current == "/" {
            //         *index -= 1;
            //         break;
            //     } else if current != "" {
            //         key_value_pair.push_str(&current);
            //     } else {
            //         break;
            //         // doesn't account for whitespace between key and value e.g., x =   {y}
            //     }
            //     *index += 1;
            // }

            if key_value_pair == "" {
                return CurrentState {
                    state:TokenizerState::Error("This should not have happened. A value was supposed to exist, but it didn't.".to_owned()),token:"".to_owned()
                };
            } else {
                return CurrentState {
                    state: TokenizerState::Props,
                    token: key_value_pair,
                };
            }
        }
    }
    fn proceed_from_component_name(markup: &Vec<char>, index: &mut usize) -> CurrentState {
        proceed_from_name(markup, index)
    }
    fn proceed_from_tag_name(markup: &Vec<char>, index: &mut usize) -> CurrentState {
        proceed_from_name(markup, index)
    }

    pub fn tokenizer(markup: String) -> impl FnMut() -> CurrentState {
        let mut current_index: usize = 0;
        let collected_markup: Vec<char> = markup.chars().collect();
        let mut state: TokenizerState = TokenizerState::Uninitialized;
        let next = move || match state {
            TokenizerState::Uninitialized => {
                // println!("current index is: {current_index}");
                let CurrentState {
                    token,
                    state: state_,
                } = proceed_from_uninitialized(&collected_markup, &mut current_index);
                state = state_;
                current_index += 1;
                return CurrentState {
                    token,
                    state: state.clone(),
                };
            }
            TokenizerState::OpenAngleBracket => {
                // println!("current index is: {current_index}");
                let CurrentState {
                    token,
                    state: state_,
                } = proceed_from_open_angle_bracket(&collected_markup, &mut current_index);
                state = state_;
                current_index += 1;
                return CurrentState {
                    token,
                    state: state.clone(),
                };
            }
            TokenizerState::Tag => {
                // println!("current index is: {current_index}");
                let CurrentState {
                    token,
                    state: state_,
                } = proceed_from_tag_name(&collected_markup, &mut current_index);
                state = state_;
                current_index += 1;
                return CurrentState {
                    token,
                    state: state.clone(),
                };
            }
            TokenizerState::Component => {
                // println!("current index is: {current_index}");
                let CurrentState {
                    token,
                    state: state_,
                } = proceed_from_component_name(&collected_markup, &mut current_index);
                state = state_;
                current_index += 1;
                return CurrentState {
                    token,
                    state: state.clone(),
                };
            }
            TokenizerState::Props => {
                let CurrentState {
                    token,
                    state: state_,
                } = proceed_from_tag_name(&collected_markup, &mut current_index);
                state = state_;
                current_index += 1;
                return CurrentState {
                    token,
                    state: state.clone(),
                };
            }
            TokenizerState::Finalized => CurrentState {
                token: "".to_owned(),
                state: state.clone(),
            },
            _ => CurrentState {
                token: "others".to_owned(),
                state: TokenizerState::Unknown,
            },
        };

        next
    }
}
