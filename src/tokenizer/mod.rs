pub mod tokenizer_mod {
    use std::borrow::{Borrow, BorrowMut};

    pub enum TokenizerCurrentState {
        Uninitialized,
        OpenAngleBracket,
        CloseAngleBracket,
        SelfClosingAngleBracket,
        Props,
        Text,
        Finalized,
        Unknown,
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
            }
        }
    }

    pub fn proceed_from_uninitialized(
        markup: &Vec<char>,
        index: i64,
    ) -> (String, TokenizerCurrentState) {
        let index: usize = index as usize;
        let max = markup.len();
        for i in index..max {
            let current_string = markup[i].to_string();
            let current = current_string.trim();
            // println!("{current}");
            if current == "<" {
                return ("<".to_owned(), TokenizerCurrentState::OpenAngleBracket);
            } else if current != "" {
                // TODO: handle error case?
                return (
                    markup.into_iter().collect(),
                    TokenizerCurrentState::Finalized,
                );
            }
        }
        ("unknown".to_owned(), TokenizerCurrentState::Unknown)
    }

    pub fn tokenizer(markup: String) -> impl FnMut() -> (String, TokenizerCurrentState) {
        // let token = String::from("");
        let current_index: i64 = 0;
        let collected_markup: Vec<char> = markup.chars().collect();
        let mut state: TokenizerCurrentState = TokenizerCurrentState::Uninitialized;

        let next = move || {
            match state {
                TokenizerCurrentState::Uninitialized => {
                    let (token, state_) =
                        proceed_from_uninitialized(&collected_markup, current_index);
                    // let a = state_.borrow();
                    state = state_;
                    return (token, state.clone());
                    // println!("token is: {tk}");

                    // read char after char, till on of the below happens:
                    // 1- reach a OpenAngleBracket
                    // 2- reach a Text -> Finalize
                }
                _ => ("others".to_owned(), TokenizerCurrentState::Unknown),
            }
        };

        next
    }
}
