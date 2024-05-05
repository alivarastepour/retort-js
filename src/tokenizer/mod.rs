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

    #[derive(Debug, Clone, PartialEq)]
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

    #[derive(Debug, PartialEq)]
    pub struct CurrentState {
        pub state: TokenizerState,
        pub token: String,
    }

    /// Determines the type of token after encountering a `<` char at uninitialized state, as it can be
    /// a ClosingTag, an OpenAngleBracket or a Text variant. `Ok` variant is returned containing the `CurrentState`
    /// if nothing goes wrong, `Err` variant explaining why otherwise.
    /// This function is responsible for advancing `index` till it reaches the char that shows the last
    /// tokenized char, which is returned in the `token` field.
    fn get_state_after_open_angle_bracket(
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
    fn proceed_from_uninitialized(
        markup: &Vec<char>,
        index: &mut usize,
    ) -> Result<CurrentState, Error> {
        let max = markup.len();
        let mut text = String::from("");
        // let mut curly_bracket_stack: Vec<String> = Vec::new();
        update_starting_tag_index(index, max, markup);
        loop {
            if *index == max {
                *index -= 1;
                let state: TokenizerState = if text == "" {
                    TokenizerState::Finalized
                } else {
                    TokenizerState::Text
                };
                let res = CurrentState {
                    state,
                    token: text.to_owned(),
                };
                return Ok(res);
            }
            let current = markup[*index].to_string();

            if current != OPEN_ANGLE_BRACKET {
                // TODO: No errors should be encountered by removing this,
                // however, we leave it as comment for now.

                // if current == OPEN_CURLY_BRACKET {
                //     curly_bracket_stack.push(OPEN_CURLY_BRACKET.to_owned());
                // } else if current == CLOSE_CURLY_BRACKET {
                //     let popped_bracket = curly_bracket_stack.pop();
                //     if popped_bracket.is_none() {
                //         let err = Error::ParsingError(
                //             "There was a parsing Error: Expected a `}`, but did not find it."
                //                 .to_owned(),
                //         );
                //         return Err(err);
                //     }
                // }
                text.push_str(&current);
            } else {
                // if curly_bracket_stack.is_empty() {
                return get_state_after_open_angle_bracket(text, index, markup);
                // } else {
                // text.push_str(&current);
                // }
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
    /// `tag_name`.This function assumes that `index` currently stands on starting
    /// character of the tag name, which is a non-whitespace character; so the caller needs to have
    /// called the `update_starting_tag_index` before calling this function.
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
    /// `Ok` variant is returned containing the `CurrentState` if nothing goes wrong,
    /// `Err` variant explaining why otherwise.
    fn get_state_after_tag_name(
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
                    let err = Error::TypeError("`get_state_after_tag_name` shouldn't have been called with this variant of `TokenizerState` as the caller; tokenizer reached a tag name without reaching `<` or `>` first.".to_owned());
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
    fn proceed_from_open_angle_bracket(
        markup: &Vec<char>,
        index: &mut usize,
        caller: TokenizerState,
    ) -> Result<CurrentState, Error> {
        let max = markup.len();
        let mut tag_name = String::from("");

        update_starting_tag_index(index, max, markup);
        if *index == max {
            *index -= 1;
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
    fn read_key_of_prop(index: &mut usize, markup: &Vec<char>) -> Result<String, Error> {
        let max = markup.len();
        let mut key = String::from("");
        loop {
            if *index == max {
                *index -= 1;
                return Err(Error::ParsingError(
                    "Expected a key-value pair, but reached the end of markup.".to_owned(),
                ));
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
        return Ok(key);
    }

    /// Returns `Ok` variant containing a String which is supposed to be the value for
    /// key-value pair of props or attributes like `alt={"This is an image"}`. `Err` variant is
    /// returned in case of errors.
    /// This function is responsible for advancing `index` till it reaches the char that shows the last
    /// tokenized char, which in this context, is supposed to be CLOSE_CURLY_BRACKET.
    fn read_value_of_prop(index: &mut usize, markup: &Vec<char>) -> Result<String, Error> {
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
                *index -= 1;
                return Err(Error::ParsingError(
                    "Expected a key-value pair, but reached the end of markup.".to_owned(),
                ));
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
    fn get_state_after_slash(
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
    fn get_state_from_props(index: &mut usize, markup: &Vec<char>) -> Result<CurrentState, Error> {
        let key_result = read_key_of_prop(index, markup);
        if key_result.is_err() {
            return Err(key_result.unwrap_err());
        }
        let key = key_result.unwrap();
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
    fn proceed_from_name(markup: &Vec<char>, index: &mut usize) -> Result<CurrentState, Error> {
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

    #[cfg(test)]

    /// Test module for `tokenizer` module's functionality.
    ///
    ///
    /// Note that it is preferred to have tests and functionality in separate modules; however, this
    /// would require to publicly interface ALL functionality of a module, which is not desired.
    mod tests {

        use super::{
            get_state_after_slash, get_state_after_tag_name, get_state_from_props,
            proceed_from_name, proceed_from_open_angle_bracket, proceed_from_uninitialized,
            read_key_of_prop, read_value_of_prop, tokenizer, update_starting_tag_index,
            update_starting_tag_name, CurrentState, TokenizerState,
        };
        use crate::error::error_mod::Error;

        #[test]
        /// An empty markup, which is any markup that has no char other than whitespace, should
        /// make tokenization state to `Finalized`.
        fn test_empty_markup() {
            let markup: Vec<char> = "          \n   \t  \n".chars().collect();
            let mut index = 0usize;
            let CurrentState { state, token } =
                proceed_from_uninitialized(&markup, &mut index).unwrap();
            match state {
                TokenizerState::Finalized => {
                    assert!(token == "" && index == markup.len() - 1)
                }
                _ => {
                    assert!(false)
                }
            }
        }

        #[test]
        fn test_text_markup() {
            let markup_string = "This is a plain test";
            let markup: Vec<char> = markup_string.chars().collect();
            let mut index = 0usize;
            let CurrentState { state, token } =
                proceed_from_uninitialized(&markup, &mut index).unwrap();
            match state {
                TokenizerState::Text => {
                    assert!(token == markup_string && index == markup.len() - 1)
                }
                _ => {
                    assert!(false)
                }
            }
        }

        #[test]
        /// When at `CurrentState::Uninitialized` and the next non-whitespace char is an open angle
        /// bracket, `CurrentState::OpenAngleBracket` must be the new state; index should be equal to
        /// index of open angle bracket char.
        fn test_open_angle_bracket() {
            let markup_string = "    <div";
            let markup: Vec<char> = markup_string.chars().collect();
            let mut index = 0usize;
            let CurrentState { state, token } =
                proceed_from_uninitialized(&markup, &mut index).unwrap();
            match state {
                TokenizerState::OpenAngleBracket => {
                    assert!(token == "<" && index == 4)
                }
                _ => {
                    assert!(false)
                }
            }
        }

        #[test]
        /// When at `CurrentState::Uninitialized` and the next non-whitespace char is an open angle
        /// bracket, `CurrentState::ClosingAngleBracket` must be the new state if there is a `/` char
        /// regardless of non-whitespace chars after it; index should be equal to index of `/` char.
        fn test_closing_angle_bracket() {
            let markup_string = "<div>hi<   /  div>";
            let markup: Vec<char> = markup_string.chars().collect();
            let mut index = 7usize;
            let CurrentState { state, token } =
                proceed_from_uninitialized(&markup, &mut index).unwrap();
            match state {
                TokenizerState::ClosingAngleBracket => {
                    assert!(token == "</" && index == 11)
                }
                _ => {
                    assert!(false)
                }
            }
        }

        #[test]
        /// When at `CurrentState::CloseAngleBracket` and the next non-whitespace char is not an open angle
        /// bracket, `CurrentState::Text` must be the new state; index should be equal to
        /// index of text's last char.
        fn test_text_inside_tag() {
            let markup_string = "<div>hello world<div>hi</div></div>";
            let markup: Vec<char> = markup_string.chars().collect();
            let mut index = 5usize;
            let CurrentState { state, token } =
                proceed_from_uninitialized(&markup, &mut index).unwrap();
            match state {
                TokenizerState::Text => {
                    assert!(token == "hello world" && index == 15)
                }
                _ => {
                    assert!(false)
                }
            }
        }

        #[test]
        /// The `update_starting_tag_index` should advance the `index` mutable reference to the first
        /// character which is not a whitespace character.
        fn test_update_starting_tag_index_from_whitespace() {
            let markup_string = "<div    \n\t   ></div>";
            let markup: Vec<char> = markup_string.chars().collect();
            let mut index = 4usize;
            update_starting_tag_index(&mut index, markup.len(), &markup);
            assert_eq!(index, 13);
        }

        #[test]
        /// The `update_starting_tag_index` should advance the `index` mutable reference to the first
        /// character which is not a whitespace character; so if `index` is already pointing to a
        /// non-whitespace character, it should not be advanced.
        fn test_update_starting_tag_index_from_non_whitespace() {
            let markup_string = "<div    \n\t   ></div>";
            let markup: Vec<char> = markup_string.chars().collect();
            let mut index = 3usize;
            update_starting_tag_index(&mut index, markup.len(), &markup);
            assert_eq!(index, 3);
        }

        #[test]
        /// the `update_starting_tag_index` should not advance the `index` to illegal state,
        /// which is more than length of markup vector.
        fn test_update_starting_tag_index_from_last_char() {
            let markup_string = "<div></div>";
            let markup: Vec<char> = markup_string.chars().collect();
            let max = markup.len();
            let mut index = max as usize;
            update_starting_tag_index(&mut index, max, &markup);
            assert_eq!(index, max);
        }

        #[test]
        /// When index is pointing to the first character of tag name, calling `update_starting_tag_name`
        /// must advance the mutable reference of `index` to the position of tag name's last character.
        fn test_update_starting_tag_name() {
            let markup_arr = vec![
                "<p id={\"hi\"}></p>".to_owned(),
                "<p></p>".to_owned(),
                "<p ></p>".to_owned(),
            ];
            let mut res: Vec<bool> = Vec::new();
            for markup_string in markup_arr {
                let markup: Vec<char> = markup_string.chars().collect();
                let mut index = 1usize;
                let mut tag_name: String = String::new();
                update_starting_tag_name(&mut index, &mut tag_name, &markup);
                res.push(index == 1 && tag_name == "p");
            }
            assert!(res.iter().all(|r| *r))
        }

        #[test]
        /// `get_state_after_tag_name` validates the argument which is passed to it as `tag_name`
        /// parameter and will return error if `tag_name` is not alphanumerical.
        fn test_get_state_after_tag_name_illegal_character() {
            let tag_name = String::from("article<");
            let result = get_state_after_tag_name(tag_name, TokenizerState::OpenAngleBracket);
            if result.is_err() {
                assert!(matches!(result.unwrap_err(), Error::ParsingError(_)));
            } else {
                assert!(false);
            }
        }

        #[test]
        /// `get_state_after_tag_name` must return a `TokenizerState::Component` variant when
        /// provided `tag_name` starts with a uppercase character.
        fn test_get_state_after_tag_name_with_component() {
            let tag_name = String::from("TableRow");
            let result =
                get_state_after_tag_name(tag_name.clone(), TokenizerState::OpenAngleBracket);
            if result.is_ok() {
                let CurrentState { state, token } = result.unwrap();
                assert!(matches!(state, TokenizerState::Component) && token == tag_name);
            } else {
                assert!(false);
            }
        }

        #[test]
        /// `get_state_after_tag_name` must return a `TokenizerState::TagNameOpen` variant when
        /// current state is `TokenizerState::OpenAngleBracket`
        fn test_get_state_after_tag_name_open() {
            let tag_name = String::from("span");
            let result =
                get_state_after_tag_name(tag_name.clone(), TokenizerState::OpenAngleBracket);
            if result.is_ok() {
                let CurrentState { state, token } = result.unwrap();
                assert!(matches!(state, TokenizerState::TagNameOpen) && token == tag_name);
            } else {
                assert!(false);
            }
        }

        #[test]
        /// `get_state_after_tag_name` must return a `TokenizerState::TagNameClose` variant when
        /// current state is `TokenizerState::ClosingAngleBracket`
        fn test_get_state_after_tag_name_close() {
            let tag_name = String::from("span");
            let result =
                get_state_after_tag_name(tag_name.clone(), TokenizerState::ClosingAngleBracket);
            if result.is_ok() {
                let CurrentState { state, token } = result.unwrap();
                assert!(matches!(state, TokenizerState::TagNameClose) && token == tag_name);
            } else {
                assert!(false);
            }
        }

        #[test]
        /// `get_state_after_tag_name` must return an error if it was called from any `TokenizerState`
        /// other than `TokenizerState::ClosingAngleBracket` and `TokenizerState::OpenAngleBracket`.
        fn test_get_state_after_tag_name_invalid_caller() {
            let tag_name = String::from("span");
            let invalid_callers: Vec<TokenizerState> = vec![
                TokenizerState::Uninitialized,
                TokenizerState::SelfClosingAngleBracket,
                TokenizerState::TagNameOpen,
                TokenizerState::TagNameClose,
                TokenizerState::Component,
                TokenizerState::Props,
                TokenizerState::Text,
                TokenizerState::Finalized,
            ];
            for caller in invalid_callers {
                let result = get_state_after_tag_name(tag_name.clone(), caller);
                if result.is_err() {
                    assert!(matches!(result.unwrap_err(), Error::TypeError(_)));
                } else {
                    assert!(false);
                }
            }
        }

        #[test]
        /// `proceed_from_open_angle_bracket` should return an `Err` variant if there is no non-empty character
        /// after `<`.
        ///
        /// Note that this is the only scenario which is checked for `proceed_from_open_angle_bracket`, because
        /// rest of its logic is basically tested. See test cases for `update_starting_tag_index`,
        /// `get_state_after_tag_name` and `update_starting_tag_name`.
        fn test_proceed_from_open_angle_bracket_empty() {
            let markup_string = "< ";
            let markup: Vec<char> = markup_string.chars().collect();
            let mut index = 1usize;
            let proceed_from_open_angle_bracket_result = proceed_from_open_angle_bracket(
                &markup,
                &mut index,
                TokenizerState::OpenAngleBracket,
            );
            assert!(matches!(
                proceed_from_open_angle_bracket_result,
                Result::Err(err) if matches!(err, Error::ParsingError(_))
            ))
        }

        #[test]
        fn test_read_key_of_prop_invalid() {
            let markup_string = "<div id";
            let markup: Vec<char> = markup_string.chars().collect();
            let mut index = 5usize;
            let read_key_of_prop_result = read_key_of_prop(&mut index, &markup);
            assert!(
                matches!(read_key_of_prop_result, Err(err) if matches!(err, Error::ParsingError(_)))
            )
        }

        #[test]
        fn test_read_key_of_prop_valid() {
            let markup_string = "<div id={\"hi\"}></div>";
            let markup: Vec<char> = markup_string.chars().collect();
            let mut index = 5usize;
            let key = read_key_of_prop(&mut index, &markup).unwrap();
            assert!(key == "id=" && index == 7);
        }

        #[test]
        /// `read_value_of_prop` expects `index` to point to a `{` character(after ignoring whitespace characters);
        /// It must return an `Err` if it's not found.
        fn test_read_value_of_prop_invalid_wrapper() {
            let markup_string = "<div id=\"hi\"></div>";
            let markup: Vec<char> = markup_string.chars().collect();
            let mut index = 8usize;
            let read_value_of_prop_result = read_value_of_prop(&mut index, &markup);
            assert!(
                matches!(read_value_of_prop_result, Err(err) if matches!(err, Error::ParsingError(_)))
            );
        }

        #[test]
        /// `read_value_of_prop` must return an `Ok` variant when it encounters a value of prop/attribute;
        /// furthermore, it should update the `index` to point to `}` character.
        fn test_read_value_of_prop() {
            let markup_string = "<div id={\"hi\"}></div>";
            let markup: Vec<char> = markup_string.chars().collect();
            let mut index = 8usize;
            let read_value_of_prop_result = read_value_of_prop(&mut index, &markup);
            if read_value_of_prop_result.is_ok() {
                let read_value_of_prop = read_value_of_prop_result.unwrap();
                assert_eq!(read_value_of_prop, "{\"hi\"}");
                assert_eq!(index, 13);
            } else {
                assert!(false);
            }
        }

        #[test]
        /// `read_value_of_prop` must return an `Err` variant when bracket sequence encounters more `{` than
        /// `}`. Note that the other way around is not handled by this function and is left off to the next
        /// state handler.
        fn test_read_value_of_prop_invalid_bracket_sequence() {
            let markup_string = "<div id={{\"hi\"}></div>";
            let markup: Vec<char> = markup_string.chars().collect();
            let mut index = 8usize;
            let read_value_of_prop_result = read_value_of_prop(&mut index, &markup);
            assert!(
                matches!(read_value_of_prop_result, Err(err) if matches!(err, Error::ParsingError(_)))
            )
        }

        #[test]
        /// `get_state_after_slash` should return an `Ok` variant if it encounters a `>` after observing a
        /// `/` character.
        fn test_get_state_after_slash_valid() {
            let markup_string = "<img / >";
            let markup: Vec<char> = markup_string.chars().collect();
            let mut index = 5usize;
            let max = markup.len();
            let get_state_after_slash_result = get_state_after_slash(&mut index, &markup, max);
            assert!(
                matches!(get_state_after_slash_result, Ok(val) if matches!(val.state, TokenizerState::SelfClosingAngleBracket) && matches!(val.token.as_str(), "/>"))
            )
        }

        #[test]
        /// `get_state_after_slash` must return an `Err` variant if no `>` is found after a `/` character.
        fn test_get_state_after_slash_invalid() {
            let markup_string = "<img / <p>hi</p>";
            let markup: Vec<char> = markup_string.chars().collect();
            let mut index = 5usize;
            let max = markup.len();
            let get_state_after_slash_result = get_state_after_slash(&mut index, &markup, max);
            assert!(
                matches!(get_state_after_slash_result, Err(err) if matches!(err, Error::ParsingError(_)))
            )
        }

        #[test]
        /// `get_state_from_props` is essentially a wrapper for `read_key_of_prop` and `read_value_of_prop`,
        /// so there is no further point in testing it thoroughly.
        fn test_get_state_from_props() {
            let markup_string = "<img src = {state.src}/>";
            let markup: Vec<char> = markup_string.chars().collect();
            let mut index = 5usize;
            let prop_result = get_state_from_props(&mut index, &markup);
            if prop_result.is_ok() {
                let CurrentState { state, token } = prop_result.unwrap();
                assert!(matches!(state, TokenizerState::Props));
                assert_eq!(token, "src={state.src}");
            } else {
                assert!(false);
            }
        }

        #[test]
        /// `proceed_from_name` should return an `Ok` which contains a `>` character when reached one
        /// after a tag's name.
        ///
        /// Note that other paths of `proceed_from_name` are covered in previous tests.
        fn test_proceed_from_name() {
            let markup_string = "<h2>Hello world</h2  >";
            let markup: Vec<char> = markup_string.chars().collect();
            let mut index = 19usize;
            let proceed_from_name_result = proceed_from_name(&markup, &mut index);
            assert!(
                matches!(proceed_from_name_result, Ok(val) if matches!(val.state, TokenizerState::CloseAngleBracket) && matches!(val.token.as_str(), ">"))
            );
        }

        // TODO: add tests for the runner every now and then.
        #[test]
        fn tokenizer_test_1() {
            let markup = String::from(
                "  <div>
            <h2>{\"User: {username}\"}</h2>
            <p>{\"Age: {age}\"}</p>
            <img src={\"https://example.com/{username}\"} alt={\"Profile picture of {username}\"} />
            <a href={\"https://example.com/{username}\"} target={\"_blank\"}>View Profile</a>
          </div>",
            );
            let mut generator = tokenizer(markup);
            loop {
                let generator_res = generator();
                assert!(!matches!(generator_res, Err(_)));
                let CurrentState {
                    state,
                    token: _token,
                } = generator_res.unwrap();
                if matches!(state, TokenizerState::Finalized) {
                    break;
                }
            }
        }

        #[test]
        fn tokenizer_test_2() {
            let markup = String::from("<div data-source={\"root\"} render-if={12+2==14}>
            <span data-source={\"hi\"}>hi</span>
              <img
              width={100} height={100} alt={2 + 2 == 4 ?   1 :0}/>
              <span style={\"color:red;font-size:2rem;font-family:sans-serif;padding:3rem\"}>hello world</span>
            </div> ");
            let expected_arr: Vec<CurrentState> = vec![
                CurrentState {
                    state: TokenizerState::OpenAngleBracket,
                    token: String::from("<"),
                },
                CurrentState {
                    state: TokenizerState::TagNameOpen,
                    token: String::from("div"),
                },
                CurrentState {
                    state: TokenizerState::Props,
                    token: String::from("data-source={\"root\"}"),
                },
                CurrentState {
                    state: TokenizerState::Props,
                    token: String::from("render-if={12+2==14}"),
                },
                CurrentState {
                    state: TokenizerState::CloseAngleBracket,
                    token: String::from(">"),
                },
                CurrentState {
                    state: TokenizerState::OpenAngleBracket,
                    token: String::from("<"),
                },
                CurrentState {
                    state: TokenizerState::TagNameOpen,
                    token: String::from("span"),
                },
                CurrentState {
                    state: TokenizerState::Props,
                    token: String::from("data-source={\"hi\"}"),
                },
                CurrentState {
                    state: TokenizerState::CloseAngleBracket,
                    token: String::from(">"),
                },
                CurrentState {
                    state: TokenizerState::Text,
                    token: String::from("hi"),
                },
                CurrentState {
                    state: TokenizerState::ClosingAngleBracket,
                    token: String::from("</"),
                },
                CurrentState {
                    state: TokenizerState::TagNameClose,
                    token: String::from("span"),
                },
                CurrentState {
                    state: TokenizerState::CloseAngleBracket,
                    token: String::from(">"),
                },
                CurrentState {
                    state: TokenizerState::OpenAngleBracket,
                    token: String::from("<"),
                },
                CurrentState {
                    state: TokenizerState::TagNameOpen,
                    token: String::from("img"),
                },
                CurrentState {
                    state: TokenizerState::Props,
                    token: String::from("width={100}"),
                },
                CurrentState {
                    state: TokenizerState::Props,
                    token: String::from("height={100}"),
                },
                CurrentState {
                    state: TokenizerState::Props,
                    token: String::from("alt={2 + 2 == 4 ?   1 :0}"),
                },
                CurrentState {
                    state: TokenizerState::SelfClosingAngleBracket,
                    token: String::from("/>"),
                },
                CurrentState {
                    state: TokenizerState::OpenAngleBracket,
                    token: String::from("<"),
                },
                CurrentState {
                    state: TokenizerState::TagNameOpen,
                    token: String::from("span"),
                },
                CurrentState {
                    state: TokenizerState::Props,
                    token: String::from(
                        "style={\"color:red;font-size:2rem;font-family:sans-serif;padding:3rem\"}",
                    ),
                },
                CurrentState {
                    state: TokenizerState::CloseAngleBracket,
                    token: String::from(">"),
                },
                CurrentState {
                    state: TokenizerState::Text,
                    token: String::from("hello world"),
                },
                CurrentState {
                    state: TokenizerState::ClosingAngleBracket,
                    token: String::from("</"),
                },
                CurrentState {
                    state: TokenizerState::TagNameClose,
                    token: String::from("span"),
                },
                CurrentState {
                    state: TokenizerState::CloseAngleBracket,
                    token: String::from(">"),
                },
                CurrentState {
                    state: TokenizerState::ClosingAngleBracket,
                    token: String::from("</"),
                },
                CurrentState {
                    state: TokenizerState::TagNameClose,
                    token: String::from("div"),
                },
                CurrentState {
                    state: TokenizerState::CloseAngleBracket,
                    token: String::from(">"),
                },
            ];
            let mut generator = tokenizer(markup);
            for expected in expected_arr {
                let actual_result = generator();
                assert!(actual_result.is_ok());
                let actual = actual_result.unwrap();
                assert!(actual == expected);
            }
        }
    }
}
