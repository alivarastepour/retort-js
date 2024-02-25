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
    let path = "x".to_owned();

    let state = String::from("{\"a\":6, \"b\":{\"c\":2,\"d\":11}}");
    let map: serde_json::Map<std::string::String, Value> = from_str(&state).unwrap();
    // let res = obj_key_path_to_value(map, path);
    // println!("res: {res}")

    // let state_object: Value = from_str(&state).unwrap;
    // match state_object {
    //     Value::Array(arr) => {
    //         println!("arr")
    //     }
    //     Value::Bool(b) => {
    //         println!("boo")
    //     }
    //     Value::Null => {
    //         println!("null")
    //     }
    //     Value::Number(num) => {
    //         println!("num")
    //     }
    //     Value::Object(obj) => {
    //         let x = into_hashamp(&obj);
    //         println!("{x}");
    //     }
    //     Value::String(st) => {
    //         println!("st")
    //     }
    // }
}
