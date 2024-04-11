pub mod util_mod {

    // TODO: remove the below functions. they are already implemented lol.
    pub fn option_has_value<T>(op: &Option<T>) -> bool {
        if let Option::None = op {
            return false;
        };
        return true;
    }

    pub fn result_is_ok<T, E>(rs: &Result<T, E>) -> bool {
        if let Result::Err(_) = rs {
            return false;
        };
        return true;
    }
}
