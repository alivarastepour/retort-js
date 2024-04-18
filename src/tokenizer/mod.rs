pub mod tokenizer_mod {
    use crate::error::error_mod::Error;

    const OPEN_ANGLE_BRACKET: &str = "<";
    const CLOSE_ANGLE_BRACKET: &str = ">";
    const FORWARD_SLASH: &str = "/";
    const SELF_CLOSING_TAG: &str = "/>";
    const CLOSING_TAG: &str = "</";
    const WHITESPACE_ALIAS: &str = "";
    const PROP_KEY_VALUE_SEPARATOR: &str = "=";
    const OPEN_CURLY_BRACKET: &str = "{";
    const CLOSE_CURLY_BRACKET: &str = "}";

    #[derive(Debug)]
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
    }

    #[derive(Debug)]
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
            }
        }
    }

    /// Determines the type of token after encountering a `<` char at uninitialized state, as it can be
    /// a ClosingTag, an OpenAngleBracket or a Text variant. `Ok` variant is returned containing the `CurrentState`
    /// if nothing goes wrong, `Err` variant explaining why otherwise.
    /// This function is responsible for advancing `index` till it reaches the char that shows the last
    /// tokenized char, which is returned in the `token` field.
    pub fn get_state_after_open_angle_bracket(
        text: String,
        index: &mut usize,
        markup: &Vec<char>,
    ) -> Result<CurrentState, Error> {
        let max = markup.len();
        if text == "" {
            let temp = index.clone(); // Cloning index helps us restore to before our assumption about the existence of `/` char.
            *index += 1;
            update_starting_tag_index(index, max, markup);
            let current_string = markup[*index].to_string();
            let current = current_string.trim();
            if current == FORWARD_SLASH {
                let res = CurrentState {
                    token: CLOSING_TAG.to_owned(),
                    state: TokenizerState::ClosingAngleBracket,
                };
                return Ok(res);
            }
            *index = temp;
            let res = CurrentState {
                state: TokenizerState::OpenAngleBracket,
                token: OPEN_ANGLE_BRACKET.to_owned(),
            };
            return Ok(res);
        } else {
            *index -= 1; // We decrement index here because it now stands on `<`, while it should stand on the last index of returned token.
            let res = CurrentState {
                state: TokenizerState::Text,
                token: text,
            };
            return Ok(res);
        }
    }

    /// Tokenize `markup` char vector starting from `index` while the current state is uninitialized.
    /// Uninitialized is used to show one of the below scenarios:
    /// 1- When tokenization has just started.
    /// 2- when tokenization has reached one of these states: TokenizerState::CloseAngleBracket,
    ///    TokenizerState::Text, TokenizerState::SelfClosingAngleBracket, and TokenizerState::Uninitialized.
    ///    This is because tokenization is dealt with in the same manner for all above states.
    /// `Ok` variant is returned containing the `CurrentState` if nothing goes wrong,
    /// `Err` variant explaining why otherwise.
    /// This function is responsible for advancing `index` till it reaches the char that shows the last
    /// tokenized char, which is returned in the `token` field.
    pub fn proceed_from_uninitialized(
        markup: &Vec<char>,
        index: &mut usize,
    ) -> Result<CurrentState, Error> {
        let max = markup.len();
        let mut text = String::from("");
        let mut curly_bracket_stack: Vec<String> = Vec::new();
        update_starting_tag_index(index, max, markup);
        loop {
            if *index == max {
                let res = CurrentState {
                    state: TokenizerState::Finalized,
                    token: "".to_owned(),
                };
                return Ok(res);
            }
            let current = markup[*index].to_string();

            if current != OPEN_ANGLE_BRACKET {
                if current == OPEN_CURLY_BRACKET {
                    curly_bracket_stack.push(OPEN_CURLY_BRACKET.to_owned());
                } else if current == CLOSE_CURLY_BRACKET {
                    let popped_bracket = curly_bracket_stack.pop();
                    if popped_bracket.is_none() {
                        let err = Error::ParsingError(
                            "There was a parsing Error: Expected a `}`, but did not find it."
                                .to_owned(),
                        );
                        return Err(err);
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
    pub fn update_starting_tag_index(index: &mut usize, max: usize, markup: &Vec<char>) {
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
    pub fn update_starting_tag_name(index: &mut usize, tag_name: &mut String, markup: &Vec<char>) {
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
    /// `Ok` variant is returned containing the `CurrentState` if nothing goes wrong,
    /// `Err` variant explaining why otherwise.
    pub fn get_state_after_tag_name(
        tag_name: String,
        caller: TokenizerState,
    ) -> Result<CurrentState, Error> {
        let collected_tag_name: Vec<char> = tag_name.chars().collect();
        let is_valid_string = collected_tag_name.iter().all(|x| x.is_alphanumeric());
        if !is_valid_string {
            let err = Error::ParsingError(format!(
                "Provided tag name `{tag_name}` contains invalid characters."
            ));
            return Err(err);
        }
        let first_letter = collected_tag_name[0];
        let is_uppercase = first_letter.is_uppercase();
        if is_uppercase {
            let res = CurrentState {
                token: tag_name,
                state: TokenizerState::Component,
            };
            return Ok(res);
        } else {
            match caller {
                TokenizerState::OpenAngleBracket => {
                    let res = CurrentState {
                        state: TokenizerState::TagNameOpen,
                        token: tag_name,
                    };
                    return Ok(res);
                }
                TokenizerState::ClosingAngleBracket => {
                    let res = CurrentState {
                        state: TokenizerState::TagNameClose,
                        token: tag_name,
                    };
                    return Ok(res);
                }
                _ => {
                    let err = Error::TypeError("This function shouldn't have been called with this variant of TokenizerState.".to_owned());
                    return Err(err);
                }
            }
        }
    }

    /// Tokenize `markup` char vector starting from `index` while the current state is OpenAngleBracket.
    /// OpenAngleBracket is used to show one of the below scenarios:
    /// 1- Encountered a '<' char which is a tag's opening; like '<div>' at index 0.
    /// 2- Encountered a '<' char which is a tag's closing: like '</div>' at index 0.
    /// This function is responsible for advancing `index` till it reaches the char that shows the last
    /// tokenized char, which is returned in the `token` field.
    /// `Ok` variant is returned containing the `CurrentState` if nothing goes wrong,
    /// `Err` variant explaining why otherwise.
    pub fn proceed_from_open_angle_bracket(
        markup: &Vec<char>,
        index: &mut usize,
        caller: TokenizerState,
    ) -> Result<CurrentState, Error> {
        let max = markup.len();
        let mut tag_name = String::from("");

        update_starting_tag_index(index, max, markup);
        if *index == max {
            let err =
                Error::ParsingError("No tag name was found after open angle bracket.".to_owned());
            return Err(err);
        }

        update_starting_tag_name(index, &mut tag_name, markup);
        return get_state_after_tag_name(tag_name, caller);
    }

    /// This function returns a String which is supposed to be a key for a key-value pair of props
    /// or attributes like `alt={"This is an image"}`.
    /// This function is responsible for advancing `index` till it reaches the char that shows the last
    /// tokenized char, which in this context, is supposed to be PROP_KEY_VALUE_SEPARATOR.
    pub fn read_key_of_prop(index: &mut usize, markup: &Vec<char>) -> String {
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

    /// Returns `Ok` variant containing a String which is supposed to be the value for
    /// key-value pair of props or attributes like `alt={"This is an image"}`. `Err` variant is
    /// returned in case of errors.
    /// This function is responsible for advancing `index` till it reaches the char that shows the last
    /// tokenized char, which in this context, is supposed to be CLOSE_CURLY_BRACKET.
    pub fn read_value_of_prop(index: &mut usize, markup: &Vec<char>) -> Result<String, Error> {
        let max = markup.len();
        update_starting_tag_index(index, max, markup);

        let value_wrapper = markup[*index].to_string();
        if value_wrapper != OPEN_CURLY_BRACKET {
            let err = Error::ParsingError(format!("Value of props and attributes must be wrapped around curly brackets. Provided char was {value_wrapper}"));
            return Err(err);
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
            let err = Error::ParsingError("Could not parse props/attributes properly. You have probably messed up with some curly brackets.".to_owned());
            return Err(err);
        }
        Ok(value)
    }

    // todo: add max to functions' parameter list

    /// Determines if the encountered `/` char is valid or not.
    /// `Ok` variant is returned containing the `CurrentState` if nothing goes wrong,
    /// `Err` variant explaining why otherwise.
    pub fn get_state_after_slash(
        index: &mut usize,
        markup: &Vec<char>,
        max: usize,
    ) -> Result<CurrentState, Error> {
        *index += 1; // we want to check if the char after `/` is `>` or not, so we must advance
                     // the index by one; otherwise, call to `update_starting_tag_index` won't advance the index
                     // because it currently stands at a non-whitespace char(/).
        update_starting_tag_index(index, max, markup);
        let has_closing_angle_bracket = markup[*index] == '>';
        if has_closing_angle_bracket {
            let res = CurrentState {
                state: TokenizerState::SelfClosingAngleBracket,
                token: SELF_CLOSING_TAG.to_owned(),
            };
            return Ok(res);
        } else {
            let err = Error::ParsingError(
                "Expected a closing angle bracket, but did not find it.".to_owned(),
            );
            return Err(err);
        }
    }

    /// Returns an `Ok` including pair of props if its format is correct, `Err` otherwise.
    /// Currently, the acceptable prop format is `key={"value"}`, `key={'value'}` and `key={js expression}`
    pub fn get_state_from_props(
        index: &mut usize,
        markup: &Vec<char>,
    ) -> Result<CurrentState, Error> {
        let key = read_key_of_prop(index, markup);
        *index += 1; // This is for PROP_KEY_VALUE_SEPARATOR
        let value_result = read_value_of_prop(index, markup);
        if value_result.is_err() {
            return Err(value_result.unwrap_err());
        }
        let value = value_result.unwrap();
        let key_value_pair = key + &value;

        if key_value_pair == "" {
            let err = Error::ParsingError(
                "This should not have happened. A value was supposed to exist, but it didn't."
                    .to_owned(),
            );
            return Err(err);
        } else {
            let res = CurrentState {
                state: TokenizerState::Props,
                token: key_value_pair,
            };
            return Ok(res);
        }
    }

    /// Tokenize `markup` char vector starting from `index` while the current state is Tag, Component or Prop.
    /// This is because after a tag name or component name or even a pair of props, we expect the same set
    /// of tokens to appear; which are CloseAngleBracket, SelfClosingAngleBracket and a pair of Props
    /// This function is responsible for advancing `index` till it reaches the char that shows the last
    /// tokenized char, which is returned in the `token` field.
    /// `Ok` variant is returned containing the `CurrentState` if nothing goes wrong,
    /// `Err` variant explaining why otherwise.
    pub fn proceed_from_name(markup: &Vec<char>, index: &mut usize) -> Result<CurrentState, Error> {
        let max = markup.len();
        update_starting_tag_index(index, max, markup);
        if markup[*index] == '>' {
            let res = CurrentState {
                token: ">".to_owned(),
                state: TokenizerState::CloseAngleBracket,
            };
            return Ok(res);
        } else if markup[*index] == '/' {
            return get_state_after_slash(index, markup, max);
        } else {
            return get_state_from_props(index, markup);
        }
    }

    // todo: how can we use DRY on the repeated behavior here?
    pub fn tokenizer(markup: String) -> impl FnMut() -> Result<CurrentState, Error> {
        let mut current_index: usize = 0;
        let collected_markup: Vec<char> = markup.chars().collect();
        let mut state: TokenizerState = TokenizerState::Uninitialized;
        let next = move || match state {
            TokenizerState::Uninitialized => {
                let current_state_result =
                    proceed_from_uninitialized(&collected_markup, &mut current_index);
                if current_state_result.is_err() {
                    return Err(current_state_result.unwrap_err());
                }
                let CurrentState {
                    state: state_,
                    token,
                } = current_state_result.unwrap();
                state = state_;
                current_index += 1;
                let res = CurrentState {
                    token,
                    state: state.clone(),
                };
                Ok(res)
            }
            TokenizerState::OpenAngleBracket => {
                let current_state_result = proceed_from_open_angle_bracket(
                    &collected_markup,
                    &mut current_index,
                    TokenizerState::OpenAngleBracket,
                );
                if current_state_result.is_err() {
                    return Err(current_state_result.unwrap_err());
                }
                let CurrentState {
                    state: state_,
                    token,
                } = current_state_result.unwrap();
                state = state_;
                current_index += 1;
                let res = CurrentState {
                    token,
                    state: state.clone(),
                };
                Ok(res)
            }
            TokenizerState::TagNameClose => {
                let current_state_result = proceed_from_name(&collected_markup, &mut current_index);
                if current_state_result.is_err() {
                    return Err(current_state_result.unwrap_err());
                }
                let CurrentState {
                    state: state_,
                    token,
                } = current_state_result.unwrap();
                state = state_;
                current_index += 1;
                let res = CurrentState {
                    token,
                    state: state.clone(),
                };
                Ok(res)
            }
            TokenizerState::TagNameOpen => {
                let current_state_result = proceed_from_name(&collected_markup, &mut current_index);
                if current_state_result.is_err() {
                    return Err(current_state_result.unwrap_err());
                }
                let CurrentState {
                    state: state_,
                    token,
                } = current_state_result.unwrap();
                state = state_;
                current_index += 1;
                let res = CurrentState {
                    token,
                    state: state.clone(),
                };
                Ok(res)
            }
            TokenizerState::Component => {
                let current_state_result = proceed_from_name(&collected_markup, &mut current_index);
                if current_state_result.is_err() {
                    return Err(current_state_result.unwrap_err());
                }
                let CurrentState {
                    state: state_,
                    token,
                } = current_state_result.unwrap();
                state = state_;
                current_index += 1;
                let res = CurrentState {
                    token,
                    state: state.clone(),
                };
                Ok(res)
            }
            TokenizerState::Props => {
                let current_state_result = proceed_from_name(&collected_markup, &mut current_index);
                if current_state_result.is_err() {
                    return Err(current_state_result.unwrap_err());
                }
                let CurrentState {
                    state: state_,
                    token,
                } = current_state_result.unwrap();
                state = state_;
                current_index += 1;
                let res = CurrentState {
                    token,
                    state: state.clone(),
                };
                Ok(res)
            }
            TokenizerState::SelfClosingAngleBracket => {
                let current_state_result =
                    proceed_from_uninitialized(&collected_markup, &mut current_index);
                if current_state_result.is_err() {
                    return Err(current_state_result.unwrap_err());
                }
                let CurrentState {
                    state: state_,
                    token,
                } = current_state_result.unwrap();
                state = state_;
                current_index += 1;
                let res = CurrentState {
                    token,
                    state: state.clone(),
                };
                Ok(res)
            }
            TokenizerState::CloseAngleBracket => {
                let current_state_result =
                    proceed_from_uninitialized(&collected_markup, &mut current_index);
                if current_state_result.is_err() {
                    return Err(current_state_result.unwrap_err());
                }
                let CurrentState {
                    state: state_,
                    token,
                } = current_state_result.unwrap();
                state = state_;
                current_index += 1;
                let res = CurrentState {
                    token,
                    state: state.clone(),
                };
                Ok(res)
            }
            TokenizerState::Text => {
                let current_state_result =
                    proceed_from_uninitialized(&collected_markup, &mut current_index);
                if current_state_result.is_err() {
                    return Err(current_state_result.unwrap_err());
                }
                let CurrentState {
                    state: state_,
                    token,
                } = current_state_result.unwrap();
                state = state_;
                current_index += 1;
                let res = CurrentState {
                    token,
                    state: state.clone(),
                };
                Ok(res)
            }
            TokenizerState::ClosingAngleBracket => {
                let current_state_result = proceed_from_open_angle_bracket(
                    &collected_markup,
                    &mut current_index,
                    TokenizerState::ClosingAngleBracket,
                );
                if current_state_result.is_err() {
                    return Err(current_state_result.unwrap_err());
                }
                let CurrentState {
                    state: state_,
                    token,
                } = current_state_result.unwrap();
                state = state_;
                current_index += 1;
                let res = CurrentState {
                    token,
                    state: state.clone(),
                };
                Ok(res)
            }
            TokenizerState::Finalized => Ok(CurrentState {
                token: "".to_owned(),
                state: state.clone(),
            }),
        };

        next
    }
}
