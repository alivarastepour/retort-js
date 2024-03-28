pub mod tokenizer_mod {
    use std::usize::MIN;

    pub enum TokenizerCurrentState {
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

    impl Clone for TokenizerCurrentState {
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

    pub fn proceed_from_uninitialized(
        markup: &Vec<char>,
        index: &mut usize,
    ) -> (String, TokenizerCurrentState) {
        let max = markup.len();
        loop {
            if *index == max {
                break;
            }
            let current_string = markup[*index].to_string();
            let current = current_string.trim();
            if current == "<" {
                return ("<".to_owned(), TokenizerCurrentState::OpenAngleBracket);
            } else if current != "" {
                // TODO: handle error case?
                return (
                    markup.into_iter().collect(),
                    TokenizerCurrentState::Finalized,
                );
            }
            *index = *index + 1;
        }
        ("".to_owned(), TokenizerCurrentState::Unknown)
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

    fn proceed_from_open_angle_bracket(
        markup: &Vec<char>,
        index: &mut usize,
    ) -> (String, TokenizerCurrentState) {
        let max = markup.len();
        let mut tag_name = String::from("");

        update_starting_tag_index(index, max, markup);
        if *index == max {
            return (
                "".to_owned(),
                TokenizerCurrentState::Error(
                    "No tag name was found after open angle bracket.".to_owned(),
                ),
            );
        }

        update_starting_tag_name(index, &mut tag_name, markup);
        let collected_tag_name: Vec<char> = tag_name.chars().collect();
        let is_valid_string = collected_tag_name.iter().all(|x| x.is_alphabetic());
        if !is_valid_string {
            return (
                "".to_owned(),
                TokenizerCurrentState::Error(
                    "Provided tag name '{tag_name}' contains invalid characters.".to_owned(),
                ),
            );
        }
        let first_letter = collected_tag_name[0];
        let is_uppercase = first_letter.is_uppercase();
        if is_uppercase {
            return (tag_name, TokenizerCurrentState::Component);
        } else {
            return (tag_name, TokenizerCurrentState::Tag);
        }
    }

    pub fn tokenizer(markup: String) -> impl FnMut() -> (String, TokenizerCurrentState) {
        let mut current_index: usize = 0;
        let collected_markup: Vec<char> = markup.chars().collect();
        let mut state: TokenizerCurrentState = TokenizerCurrentState::Uninitialized;
        let next = move || match state {
            TokenizerCurrentState::Uninitialized => {
                println!("current index is: {current_index}");
                let (token, state_) =
                    proceed_from_uninitialized(&collected_markup, &mut current_index);
                state = state_;
                current_index += 1;
                return (token, state.clone());
            }
            TokenizerCurrentState::OpenAngleBracket => {
                println!("current index is: {current_index}");
                let (token, state_) =
                    proceed_from_open_angle_bracket(&collected_markup, &mut current_index);
                state = state_;
                current_index += 1;
                return (token, state.clone());
            }
            TokenizerCurrentState::Finalized => ("".to_owned(), state.clone()),
            _ => ("others".to_owned(), TokenizerCurrentState::Unknown),
        };

        next
    }
}
