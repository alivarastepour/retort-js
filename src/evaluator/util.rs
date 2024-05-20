/// Contains utility functions used in the process of evaluation
pub mod evaluator_util {

    /// performs a look ahead search on the validity of JS expression. This requires using
    /// escape chars for special chars.
    pub fn has_valid_expression_inside(text: String) -> bool {
        let open_c = text.matches("{").count();
        let close_c = text.matches("}").count();
        return open_c == close_c && open_c > 0;
    }

    /// Given a trimmed string input, checks whether it is wrapped inside curly brackets or not.
    pub fn is_a_valid_attribute_value(text: &str) -> bool {
        text.starts_with("{") && text.ends_with("}")
    }

    /// Returns true if `text` is convertible to number.
    pub fn attribute_value_is_number(text: &str) -> bool {
        text.parse::<i64>().is_ok()
    }

    /// Returns true if `text` is convertible to bool.
    pub fn attribute_value_is_bool(text: &str) -> bool {
        text.parse::<bool>().is_ok()
    }

    /// Returns true if `text` is wrapped inside quotation marks.
    pub fn attribute_value_is_wrapped_in_quotes(text: &str) -> bool {
        (text.starts_with("\"") && text.ends_with("\""))
            || (text.starts_with("'") && text.ends_with("'"))
    }

    #[cfg(test)]
    mod tests {
        use crate::evaluator::util::evaluator_util::*;

        #[test]
        /// `has_valid_expression_inside` must return true when provided argument fits to the defined
        /// format for expressions.
        fn test_has_valid_expression_inside() {
            let expression = String::from("hello-{state.name}");
            let has_valid_expression_inside_result = has_valid_expression_inside(expression);
            assert!(has_valid_expression_inside_result);
        }

        #[test]
        #[ignore = "https://github.com/alivarastepour/retort-js/issues/42"]
        /// `has_valid_expression_inside` should return false for values which are String and
        /// not Expression.
        fn test_no_valid_expression_inside() {
            let expression = String::from("some random text");
            let has_valid_expression_inside_result = has_valid_expression_inside(expression);
            assert!(!has_valid_expression_inside_result);
        }

        #[test]
        #[ignore = "https://github.com/alivarastepour/retort-js/issues/42"]
        /// `has_valid_expression_inside` should ignore escaped curly brackets.
        fn test_no_valid_expression_inside_with_curly_brackets() {
            let expression = String::from("some text with arbitrary \u{007D} and \u{007B}");
            let has_valid_expression_inside_result = has_valid_expression_inside(expression);
            assert!(!has_valid_expression_inside_result);
        }

        #[test]
        /// `is_a_valid_attribute_value` must return true when value is wrapped between curly brackets.
        fn test_is_a_valid_attribute_value() {
            assert!(is_a_valid_attribute_value("{callback}"));
        }

        #[test]
        /// `is_a_valid_attribute_value` must return false when value doesn't fit into defined patterns,
        /// even if it is accepted in JS itself.
        fn test_is_not_a_valid_attribute_value() {
            assert!(!is_a_valid_attribute_value("`value-${id}`"));
        }

        #[test]
        /// `attribute_value_is_number` must return true when argument is parsable to number.
        fn test_attribute_value_is_number() {
            assert!(attribute_value_is_number("12"));
        }

        #[test]
        /// `attribute_value_is_number` must return false when argument is not parsable to number.
        fn test_attribute_value_is_not_number() {
            assert!(!attribute_value_is_number(" 12"));
        }

        #[test]
        /// `attribute_value_is_bool` must return true when argument is parsable to boolean, i.e., is equal
        /// to string "true" of "false", case sensitive.
        fn test_attribute_value_is_bool() {
            assert!(attribute_value_is_bool("true"));
        }

        #[test]
        /// `attribute_value_is_bool` must return false when argument is not parsable to boolean.
        fn test_attribute_value_is_not_bool() {
            assert!(!attribute_value_is_bool("False"));
        }

        #[test]
        /// `attribute_value_is_wrapped_in_quotes` must return true when it is wrapped between allowed
        /// quotation marks, i.e., `"` and `'`.
        fn test_attribute_value_is_wrapped_in_quotes() {
            assert!(attribute_value_is_wrapped_in_quotes("'hi:('"));
        }

        #[test]
        /// `attribute_value_is_wrapped_in_quotes` must return false when it is not wrapped between allowed
        /// quotation marks.
        fn test_attribute_value_is_not_wrapped_in_quotes() {
            assert!(!attribute_value_is_wrapped_in_quotes("\"hi:('"));
        }
    }
}
