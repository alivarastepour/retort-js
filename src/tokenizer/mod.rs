pub mod tokenizer_mod {

    const OPEN_ANGLE_BRACKET: &str = "<";
    const CLOSE_ANGLE_BRACKET: &str = ">";
    const FORWARD_SLASH: &str = "/";
    const SELF_CLOSING_TAG: &str = "/>";
    const CLOSING_TAG: &str = "</";
    const WHITESPACE_ALIAS: &str = "";
    const PROP_KEY_VALUE_SEPARATOR: &str = "=";
    const OPEN_CURLY_BRACKET: &str = "{";
    const CLOSE_CURLY_BRACKET: &str = "}";
    pub enum TokenizerState {
        Uninitialized,
        OpenAngleBracket,        // <
        CloseAngleBracket,       // >
        SelfClosingAngleBracket, // />
        ClosingAngleBracket,     // </
        TagNameOpen,
        TagNameClose,
        Component,
        Props,
        Text,
        Finalized,
        Error(String),
    }

    pub struct CurrentState {
        pub state: TokenizerState,
        pub token: String,
    }

    impl Clone for TokenizerState {
        fn clone(&self) -> Self {
            match self {
                Self::Uninitialized => Self::Uninitialized,
                Self::OpenAngleBracket => Self::OpenAngleBracket,
                Self::SelfClosingAngleBracket => Self::SelfClosingAngleBracket,
                Self::ClosingAngleBracket => Self::ClosingAngleBracket,
                Self::CloseAngleBracket => Self::CloseAngleBracket,
                Self::TagNameOpen => Self::TagNameOpen,
                Self::TagNameClose => Self::TagNameClose,
                Self::Props => Self::Props,
                Self::Text => Self::Text,
                Self::Finalized => Self::Finalized,
                Self::Component => Self::Component,
                Self::Error(err) => Self::Error(err.clone()),
            }
        }
    }

    /// Determines the type of token after encountering a `<` char at uninitialized state, as it can be
    /// a ClosingTag, an OpenAngleBracket or a Text variant.
    /// This function is responsible for advancing `index` till it reaches the char that shows the last
    /// tokenized char, which is returned in the `token` field.
    fn get_state_after_open_angle_bracket(
        text: String,
        index: &mut usize,
        markup: &Vec<char>,
    ) -> CurrentState {
        let max = markup.len();
        if text == "" {
            let temp = index.clone(); // Cloning index helps us restore to before our assumption about the existence of `/` char.
            *index += 1;
            update_starting_tag_index(index, max, markup);
            let current_string = markup[*index].to_string();
            let current = current_string.trim();
            if current == FORWARD_SLASH {
                return CurrentState {
                    token: CLOSING_TAG.to_owned(),
                    state: TokenizerState::ClosingAngleBracket,
                };
            }
            *index = temp;
            return CurrentState {
                state: TokenizerState::OpenAngleBracket,
                token: OPEN_ANGLE_BRACKET.to_owned(),
            };
        } else {
            *index -= 1; // We decrement index here because it now stands on `<`, while it should stand on the last index of returned token.
            return CurrentState {
                state: TokenizerState::Text,
                token: text,
            };
        }
    }

    /// Tokenize `markup` char vector starting from `index` while the current state is uninitialized.
    /// Uninitialized is used to show one of the below scenarios:
    /// 1- When tokenization has just started.
    /// 2- when tokenization has reached one of these states: TokenizerState::CloseAngleBracket,
    ///    TokenizerState::Text, TokenizerState::SelfClosingAngleBracket, and TokenizerState::Uninitialized.
    ///    This is because tokenization is dealt with in the same manner for all above states.
    /// This function is responsible for advancing `index` till it reaches the char that shows the last
    /// tokenized char, which is returned in the `token` field.
    pub fn proceed_from_uninitialized(markup: &Vec<char>, index: &mut usize) -> CurrentState {
        let max = markup.len();
        let mut text = String::from("");
        let mut curly_bracket_stack: Vec<String> = Vec::new();
        update_starting_tag_index(index, max, markup);
        loop {
            if *index == max {
                return CurrentState {
                    state: TokenizerState::Finalized,
                    token: "".to_owned(),
                };
            }
            let current = markup[*index].to_string();

            if current != OPEN_ANGLE_BRACKET {
                if current == OPEN_CURLY_BRACKET {
                    curly_bracket_stack.push(OPEN_CURLY_BRACKET.to_owned());
                } else if current == CLOSE_CURLY_BRACKET {
                    let popped_bracket = curly_bracket_stack.pop();
                    if popped_bracket.is_none() {
                        return CurrentState {
                            state: TokenizerState::Error(
                                "There was a parsing Error. Expected a `}`, but did not find it."
                                    .to_owned(),
                            ),
                            token: "".to_owned(),
                        };
                    }
                }
                text.push_str(&current);
            } else {
                if curly_bracket_stack.is_empty() {
                    return get_state_after_open_angle_bracket(text, index, markup);
                } else {
                    text.push_str(&current);
                }
            }
            *index = *index + 1;
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
            if current != WHITESPACE_ALIAS {
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
            if current != WHITESPACE_ALIAS && current != CLOSE_ANGLE_BRACKET {
                tag_name.push_str(&current);
                *index += 1;
            } else {
                *index -= 1; // `index` is decremented because we now stand at a whitespace alias char or `>`; but index must point to the last char of tag's name.
                break;
            }
        }
    }

    /// Determines the type of token after tag's name is built.
    fn get_state_after_tag_name(tag_name: String, caller: bool) -> CurrentState {
        let collected_tag_name: Vec<char> = tag_name.chars().collect();
        let is_valid_string = collected_tag_name.iter().all(|x| x.is_alphanumeric());
        if !is_valid_string {
            return CurrentState {
                token: "".to_owned(),
                state: TokenizerState::Error(
                    format!("Provided tag name `{tag_name}` contains invalid characters.")
                        .to_owned(),
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
            let state = if caller {
                TokenizerState::TagNameOpen
            } else {
                TokenizerState::TagNameClose
            };
            return CurrentState {
                token: tag_name,
                state,
            };
        }
    }

    /// Tokenize `markup` char vector starting from `index` while the current state is OpenAngleBracket.
    /// OpenAngleBracket is used to show one of the below scenarios:
    /// 1- Encountered a '<' char which is a tag's opening; like '<div>' at index 0.
    /// 2- Encountered a '<' char which is a tag's closing: like '</div>' at index 0.
    /// This function is responsible for advancing `index` till it reaches the char that shows the last
    /// tokenized char, which is returned in the `token` field.
    fn proceed_from_open_angle_bracket(
        markup: &Vec<char>,
        index: &mut usize,
        caller: bool,
    ) -> CurrentState {
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
        return get_state_after_tag_name(tag_name, caller);
    }

    /// This function returns a String which is supposed to be a key for a key-value pair of props
    /// or attributes like `alt={"This is an image"}`.
    /// This function is responsible for advancing `index` till it reaches the char that shows the last
    /// tokenized char, which in this context, is supposed to be PROP_KEY_VALUE_SEPARATOR.
    fn read_key_of_prop(index: &mut usize, markup: &Vec<char>) -> String {
        let max = markup.len();
        let mut key = String::from("");
        loop {
            if *index == max {
                break;
            }
            let mut current = markup[*index].to_string(); // TODO: generalize this shit
            current = current.trim().to_owned();
            if current != WHITESPACE_ALIAS {
                key.push_str(&current);
            }
            if current == PROP_KEY_VALUE_SEPARATOR {
                break;
            }
            *index += 1;
        }
        return key;
    }

    /// This function returns a String which is supposed to be the value for  key-value pair of props
    /// or attributes like `alt={"This is an image"}`.
    /// This function is responsible for advancing `index` till it reaches the char that shows the last
    /// tokenized char, which in this context, is supposed to be CLOSE_CURLY_BRACKET.
    fn read_value_of_prop(index: &mut usize, markup: &Vec<char>) -> String {
        let max = markup.len();
        update_starting_tag_index(index, max, markup);

        let value_wrapper = markup[*index].to_string();
        if value_wrapper != OPEN_CURLY_BRACKET {
            panic!("Value of props and attributes must be wrapped around curly braces. Provided char was {value_wrapper}")
        }
        let mut value = String::from("");
        let mut wrapper_stack: Vec<String> = Vec::new();
        loop {
            if *index == max {
                break;
            }
            let mut current = markup[*index].to_string(); // todo: generalize this shit
            current = current.to_owned();
            value.push_str(&current);
            if current == OPEN_CURLY_BRACKET {
                wrapper_stack.push(current);
            } else if current == CLOSE_CURLY_BRACKET {
                wrapper_stack.pop();
            }
            if wrapper_stack.is_empty() {
                break;
            }

            *index += 1;
        }
        if !wrapper_stack.is_empty() {
            panic!("Could not parse props/attributes properly. You have probably messed up with some curly brackets.");
        }
        value
    }

    // todo: add max to functions' parameter list

    /// Determines if the encountered `/` char is valid or not
    fn get_state_after_slash(index: &mut usize, markup: &Vec<char>, max: usize) -> CurrentState {
        *index += 1; // we want to check if the char after `/` is `>` or not, so we must advance
                     // the index by one; otherwise, call to `update_starting_tag_index` won't advance the index
                     // because it currently stands at a non-whitespace char(/).
        update_starting_tag_index(index, max, markup);
        let has_closing_angle_bracket = markup[*index] == '>';
        if has_closing_angle_bracket {
            CurrentState {
                state: TokenizerState::SelfClosingAngleBracket,
                token: SELF_CLOSING_TAG.to_owned(),
            }
        } else {
            CurrentState {
                state: TokenizerState::Error(
                    "Expected a closing angle bracket, but did not find it.".to_owned(),
                ),
                token: "".to_owned(),
            }
        }
    }

    /// Returns a pair of props if its format is correct, error otherwise.
    /// Currently, the acceptable prop format is `key={"value"}`, `key={'value'}` and `key={js expression}`
    fn get_state_from_props(index: &mut usize, markup: &Vec<char>) -> CurrentState {
        let key = read_key_of_prop(index, markup);
        *index += 1; // This is for PROP_KEY_VALUE_SEPARATOR
        let value = read_value_of_prop(index, markup);
        let key_value_pair = key + &value;

        if key_value_pair == "" {
            return CurrentState {
                state: TokenizerState::Error(
                    "This should not have happened. A value was supposed to exist, but it didn't."
                        .to_owned(),
                ),
                token: "".to_owned(),
            };
        } else {
            return CurrentState {
                state: TokenizerState::Props,
                token: key_value_pair,
            };
        }
    }

    /// Tokenize `markup` char vector starting from `index` while the current state is Tag, Component or Prop.
    /// This is because after a tag name or component name or even a pair of props, we expect the same set
    /// of tokens to appear; which are CloseAngleBracket, SelfClosingAngleBracket and a pair of Props
    /// This function is responsible for advancing `index` till it reaches the char that shows the last
    /// tokenized char, which is returned in the `token` field.
    fn proceed_from_name(markup: &Vec<char>, index: &mut usize) -> CurrentState {
        let max = markup.len();
        update_starting_tag_index(index, max, markup);
        if markup[*index] == '>' {
            CurrentState {
                token: ">".to_owned(),
                state: TokenizerState::CloseAngleBracket,
            }
        } else if markup[*index] == '/' {
            return get_state_after_slash(index, markup, max);
        } else {
            return get_state_from_props(index, markup);
        }
    }

    pub fn tokenizer(markup: String) -> impl FnMut() -> CurrentState {
        let mut current_index: usize = 0;
        let collected_markup: Vec<char> = markup.chars().collect();
        let mut state: TokenizerState = TokenizerState::Uninitialized;
        let next = move || match state {
            TokenizerState::Uninitialized => {
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
                let CurrentState {
                    token,
                    state: state_,
                } = proceed_from_open_angle_bracket(&collected_markup, &mut current_index, true);
                state = state_;
                current_index += 1;
                return CurrentState {
                    token,
                    state: state.clone(),
                };
            }
            TokenizerState::TagNameClose => {
                let CurrentState {
                    token,
                    state: state_,
                } = proceed_from_name(&collected_markup, &mut current_index);
                state = state_;
                current_index += 1;
                return CurrentState {
                    token,
                    state: state.clone(),
                };
            }
            TokenizerState::TagNameOpen => {
                let CurrentState {
                    token,
                    state: state_,
                } = proceed_from_name(&collected_markup, &mut current_index);
                state = state_;
                current_index += 1;
                return CurrentState {
                    token,
                    state: state.clone(),
                };
            }
            TokenizerState::Component => {
                let CurrentState {
                    token,
                    state: state_,
                } = proceed_from_name(&collected_markup, &mut current_index);
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
                } = proceed_from_name(&collected_markup, &mut current_index);
                state = state_;
                current_index += 1;
                return CurrentState {
                    token,
                    state: state.clone(),
                };
            }
            TokenizerState::SelfClosingAngleBracket => {
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
            TokenizerState::CloseAngleBracket => {
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
            TokenizerState::Text => {
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
            TokenizerState::ClosingAngleBracket => {
                let CurrentState {
                    token,
                    state: state_,
                } = proceed_from_open_angle_bracket(&collected_markup, &mut current_index, false);
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
                token: "".to_owned(),
                state: state.clone(),
            },
        };

        next
    }
}
