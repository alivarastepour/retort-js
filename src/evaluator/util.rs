/// Contains utility functions used in the process of evaluation
pub mod evaluator_util {

    /// performs a look ahead search on the validity of JS expression. This requires using
    /// escape chars for special chars.
    pub fn has_valid_expression_inside(text: String) -> bool {
        let open_c = text.matches("{").count();
        let close_c = text.matches("}").count();
        return open_c == close_c;
    }

    /// Performs a check on the wrapper chars of value of the attributes'.
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
}
