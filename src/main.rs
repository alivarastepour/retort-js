mod file_util;
use crate::file_util::file_util_mod;
use std::env;

fn main() {
    let val = env::current_dir().unwrap();
    let current = val.to_str().unwrap();
    let path = format!("{current}/test/HelloWorld/presenter.rtjs");
    let a = file_util_mod::read_file(&path);
    let b = a.imports;
    let c = a.markup;

    println!("{c}");
}
