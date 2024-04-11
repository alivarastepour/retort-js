pub mod test_mod {
    #[cfg(test)]
    mod tokenizer_mod_test {
        use crate::tokenizer::tokenizer_mod::{
            proceed_from_uninitialized, CurrentState, TokenizerState,
        };
        #[test]
        fn test_empty_markup() {
            let markup: Vec<char> = " ".chars().collect();
            let mut index = 0usize;
            let CurrentState { state, token } = proceed_from_uninitialized(&markup, &mut index);
            match state {
                TokenizerState::Finalized => {
                    assert!(token == "")
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
            let CurrentState { state, token } = proceed_from_uninitialized(&markup, &mut index);
            match state {
                TokenizerState::Finalized => {
                    assert!(token == markup_string)
                }
                _ => {
                    assert!(false)
                }
            }
        }
    }
}
