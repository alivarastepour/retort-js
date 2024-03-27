mod file_util;
use crate::{file_util::file_util_mod, tokenizer::tokenizer_mod::TokenizerCurrentState};
use std::env;
mod tokenizer;

use tokenizer::tokenizer_mod;

fn main() {
    // let val = env::current_dir().unwrap();
    // let current = val.to_str().unwrap();
    // let path = format!("{current}/test/HelloWorld/presenter.rtjs");
    // let a = file_util_mod::read_file(&path);
    // let b = a.imports;
    // let c = a.markup;

    // let r = " ".to_owned();
    // let s = r.trim();
    // let a = s == "";
    // print!("{a}");

    let markup = "
    

<div className=\"hi\" id=\"x\">hello world!</div>

<ByeWorld xprop=\"12\" some={value % 2}/>

    "
    .to_owned();

    let mut next = tokenizer_mod::tokenizer(markup);

    loop {
        let (token, state) = next();
        println!("token is: {token}");
        if let TokenizerCurrentState::Unknown = state {
            break;
        }
    }

    // println!("{c}");
}
