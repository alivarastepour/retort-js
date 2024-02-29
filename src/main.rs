use regex::Regex;
use serde_json::{from_str, Map, Value};

// fn into_hashamp(map: &serde_json::Map<std::string::String, Value>) -> HashMap<String, String> {
//     let res: HashMap<String, String> = HashMap::new();
//     for (k, v) in map {
//         match v {
//             // Value::
//         }
//         res.insert(k.to_owned(), v.to_owned());
//     }
//     res
// }

fn main() {
    let markup = "<div onclick={someShit}>I am {age} years old</div>";
    let markup_regex = Regex::new(">.*\\{(.*)\\}.*<").unwrap();
    // let what = markup_regex.captures_iter(markup);
    // let stuff = Regex::find(&markup_regex, markup).unwrap();
    // let x = Regex::captures_iter(&markup_regex, markup);
    // for item in x {
    //     let ha: (&str, [&str; 1]) = item.extract();
    //     let h1 = ha.0;
    //     let x = ha.1[0];
    //     println!("{x}")
    // }
    let a = markup_regex.find(markup).unwrap();
    println!("{:?}", a)
}
