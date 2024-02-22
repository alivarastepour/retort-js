use std::iter;

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

fn unwrap_value(value: &Value) -> Value {
    match value {
        Value::String(obj) => Value::String(obj.to_owned()),
        Value::Object(obj) => Value::Object(obj.to_owned()),
        _ => Value::Null,
    }
}

fn obj_key_path_to_value(map: Map<String, Value>, path: String) -> String {
    let mut iterator = path.split('.');
    let iterator_vec: &Vec<&str> = &iterator.collect();
    let len = iterator_vec.len();
    let mut a = 1;
    // while let val = iterator.next() {
    //     match val {
    //         Option::None => {}
    //         Option::Some(v) => {
    //             println!("{a} : {v}")
    //         }
    //     }
    // }
    loop {
        let val = iterator.next();
        match val {
            Option::None => {
                break;
            }
            Option::Some(v) => {
                let map_item = map.get(v);
                match map_item {
                    Option::None => {}
                    Option::Some(value_pair) => {}
                }
            }
        }
    }
    // path : a.b.c.d...z
}

fn main() {
    println!("Hello, world!");
    let path = "a.b.c.d.e".to_owned();
    obj_key_path_to_value(Map::new(), path);

    // let state = String::from("{\"a\":2, \"b\":{\"c\":2,\"d\":11}}");

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
