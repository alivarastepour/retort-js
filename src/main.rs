// fn extract_expression_and_string(string_with_expression: String) -> Result<Vec<String>, ()> {
//     let string_with_expression_chars: Vec<char> = string_with_expression.chars().collect();
//     let mut result: Vec<String> = Vec::new();
//     let mut expression_stack = Vec::new();
//     let mut current_expression: String = String::new();
//     let mut current_string: String = String::new();
//     for chr in string_with_expression_chars {
//         if chr == '{' {
//             expression_stack.push('{');
//             result.push(current_string.clone());
//             current_string.clear();
//             current_expression += "{";
//         } else if chr == '}' {
//             let head = expression_stack.pop();
//             if head.is_none() {
//                 println!("???");
//                 return Err(());
//             }
//             current_expression += &chr.to_string();
//             if expression_stack.is_empty() {
//                 result.push(current_expression.clone());
//                 current_expression.clear();
//             }
//         } else {
//             if expression_stack.is_empty() {
//                 current_string += &chr.to_string();
//             } else {
//                 current_expression += &chr.to_string();
//             }
//         }
//     }
//     if !current_string.is_empty() {
//         result.push(current_string);
//     }
//     Ok(result)
// }

// use fancy_regex::Regex;

// fn main() {
//     let exp = "{state.value > 4 ? <span>xx</span> : <p>{state.value > 2 ? <span>aa</span> : <p>b</p>}</p>}".to_owned();
//     // let pattern = r#"<([^>]*)(?:(?:>(.*?)<\/\1>)|(?:(?!<\/\1>|<[^>]*>).)*)(?:<\/\1>)?"#;
//     let re = Regex::new(r#"<(\w+)[^>]*>(.*?)</\1>"#).unwrap();

//     // let re = Regex::new(pattern).unwrap();
//     for mat in re.find_iter(&exp) {
//         if mat.is_err() {
//             continue;
//         }
//         let x = mat.unwrap();
//         let (start, end) = (x.start(), x.end());
//         let matched_a = &exp[start..end];
//         println!("{matched_a}");
//     }
// }

fn main() {}
