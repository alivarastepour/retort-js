pub mod const_util_mod {
    pub const APP_WRAPPER_ID: &str = "root";
    pub const RENDER_IF_ATTRIBUTE_NAME: &str = "render-if";
    pub const RENDER_ELSE_IF_ATTRIBUTE_NAME: &str = "render-else-if";
    pub const RENDER_ELSE_ATTRIBUTE_NAME: &str = "render-else";
    pub const DOM_ERROR: &str = "DOM error";
    pub const PARSING_ERROR: &str = "Parsing error";
    pub const REFERENCE_ERROR: &str = "Reference error";
    pub const EVALUATION_ERROR: &str = "Evaluation error";
    pub const TYPE_ERROR: &str = "Type error";
    pub const _INVESTIGATION_NEEDED_ERROR: &str = "Unknown error";
    pub const SERDE_WASM_BINDGEN_ERROR: &str = "Serialization error";
    pub const RESOLVE_ERROR: &str = "Resolve error";
    pub const ERROR_WRAPPER_STYLES: &str = "
                                            line-height:30px;
                                            background-color:#570606;
                                            font-family:sans-serif;
                                            font-size:1.5rem;
                                            padding:2rem;
                                            color:#fff;
                                            font-weight:700;
                                            border-radius:8px
                                            ";
    pub const ERROR_SUBTITLE_STYLES: &str = "
                                            color:#eee;
                                            font-size:0.8rem;
                                            font-family:sans-serif
                                            ";
    pub const ERROR_SUBTITLE: &str = "See console for more details.";
    pub const USE_STRICT: &str = "\"use strict\";";
    pub const STATE_PARAMETER: &str = "state_";
    pub const PROPS_PARAMETER: &str = "props_";
    /// In JSON strings which contain arrays, `stringify` method is called twice
    /// when converting. Since we need to call the `parse` as many times as we have called the `stringify`,
    /// we must check whether the type of state is `object` or not after the first call to `parse`.
    pub const CLOSURE: &str =
        "let state=JSON.parse(state_);if(typeof state === 'string'){state=JSON.parse(state)}";
    pub const RETURN: &str = "return ";
    pub const UNDEFINED_LITERAL: &str = "undefined";
    pub const NULL_LITERAL: &str = "null";
    pub const OPEN_CURLY_BRACKET: char = '{';
    pub const CLOSE_CURLY_BRACKET: char = '}';
    pub const OPEN_ANGLE_BRACKET: char = '<';
    pub const CLOSE_ANGLE_BRACKET: char = '>';
    const WHITESPACE_ALIAS: &str = "";
    const FORWARD_SLASH: &str = "/";
    pub const SELF_CLOSING_TAG: &str = "/>";
    pub const CLOSING_TAG: &str = "</";
    pub const ATTRIBUTE_KEY_VALUE_SEPARATOR: &str = "=";
    pub const IMPORT_KEYWORD: &str = "import";
    const FROM_KEYWORD: &str = "from";
    const TRUE_LITERAL: &str = "true";

    /// returns true if the `input` parameter is equal to predefined `TRUE_LITERAL` constant.
    pub fn is_input_true_literal(input: &str) -> bool {
        return input == TRUE_LITERAL;
    }

    /// returns true if the `input` parameter is equal to predefined `UNDEFINED_LITERAL` constant.
    pub fn is_input_undefined_literal(input: &str) -> bool {
        return input == UNDEFINED_LITERAL;
    }

    /// returns true if the `input` parameter is equal to predefined `NULL_LITERAL` constant.
    pub fn is_input_null_literal(input: &str) -> bool {
        return input == NULL_LITERAL;
    }

    /// returns true if the `input` parameter is equal to predefined `OPEN_CURLY_BRACKET` constant.
    pub fn is_input_open_curly_bracket(input: char) -> bool {
        return input == OPEN_CURLY_BRACKET;
    }

    /// returns true if the `input` parameter is equal to predefined `CLOSE_CURLY_BRACKET` constant.
    pub fn is_input_close_curly_bracket(input: char) -> bool {
        return input == CLOSE_CURLY_BRACKET;
    }

    /// returns true if the `input` parameter is equal to predefined `CLOSE_ANGLE_BRACKET` constant.
    pub fn is_input_close_angle_bracket(input: char) -> bool {
        return input == CLOSE_ANGLE_BRACKET;
    }

    /// returns true if the `input` parameter is equal to predefined `CLOSE_ANGLE_BRACKET` constant.
    pub fn is_input_open_angle_bracket(input: char) -> bool {
        return input == OPEN_ANGLE_BRACKET;
    }

    /// returns true if the `input` parameter is equal to predefined `WHITESPACE_ALIAS` constant.
    pub fn is_input_white_space_alias(input: &str) -> bool {
        return input == WHITESPACE_ALIAS;
    }

    /// returns true if the `input` parameter is equal to predefined `ATTRIBUTE_KEY_VALUE_SEPARATOR` constant.
    pub fn is_input_attribute_key_value_separator(input: &str) -> bool {
        return input == ATTRIBUTE_KEY_VALUE_SEPARATOR;
    }

    /// returns true if the `input` parameter is equal to predefined `FORWARD_SLASH` constant.
    pub fn is_input_forward_slash(input: &str) -> bool {
        return input == FORWARD_SLASH;
    }

    pub fn is_input_import_keyword(input: &str) -> bool {
        return input == IMPORT_KEYWORD;
    }

    pub fn is_input_from_keyword(input: &str) -> bool {
        return input == FROM_KEYWORD;
    }
}
