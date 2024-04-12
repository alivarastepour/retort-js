fn main() {
    let a = "1".parse::<bool>();
    match a {
        Result::Ok(val) => {
            println!("OK: {val}")
        }
        Result::Err(err) => {
            println!("Err: {err}")
        }
    }
}
