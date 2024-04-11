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
        #[ignore = "known issue: https://github.com/alivarastepour/retort-js/issues/5"]
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

        #[test]
        fn test_open_angle_bracket() {
            let markup_string = "<div";
            let markup: Vec<char> = markup_string.chars().collect();
            let mut index = 0usize;
            let CurrentState { state, token } = proceed_from_uninitialized(&markup, &mut index);
            match state {
                TokenizerState::OpenAngleBracket => {
                    assert!(token == "<")
                }
                _ => {
                    assert!(false)
                }
            }
        }

        #[test]
        fn test_open_angle_bracket_with_extra_spaces() {
            let markup_string = "   <div";
            let markup: Vec<char> = markup_string.chars().collect();
            let mut index = 0usize;
            let CurrentState { state, token } = proceed_from_uninitialized(&markup, &mut index);
            match state {
                TokenizerState::OpenAngleBracket => {
                    assert!(token == "<")
                }
                _ => {
                    assert!(false)
                }
            }
        }

        #[test]
        fn test_closing_angle_bracket() {
            let markup_string = "<div>hi</div>";
            let markup: Vec<char> = markup_string.chars().collect();
            let mut index = 7usize;
            let CurrentState { state, token } = proceed_from_uninitialized(&markup, &mut index);
            match state {
                TokenizerState::ClosingAngleBracket => {
                    assert!(token == "</")
                }
                _ => {
                    assert!(false)
                }
            }
        }

        #[test]
        fn test_closing_angle_bracket_with_extra_spaces() {
            let markup_string = "<div>hi<      /div>";
            let markup: Vec<char> = markup_string.chars().collect();
            let mut index = 7usize;
            let CurrentState { state, token } = proceed_from_uninitialized(&markup, &mut index);
            match state {
                TokenizerState::ClosingAngleBracket => {
                    assert!(token == "</")
                }
                _ => {
                    assert!(false)
                }
            }
        }

        #[test]
        #[ignore = "provided method does not capture the desired behavior. we may have to test this somewhere else."]
        fn test_text_inside_tag() {
            let markup_string = "<div>hello world<div>hi</div></div>";
            let markup: Vec<char> = markup_string.chars().collect();
            let mut index = 16usize;
            let CurrentState { state, token } = proceed_from_uninitialized(&markup, &mut index);
            match state {
                TokenizerState::Text => {
                    assert!(token == "hello world")
                }
                _ => {
                    assert!(false)
                }
            }
        }
    }
}
