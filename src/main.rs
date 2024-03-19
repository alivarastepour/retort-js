use scraper::{Html, Selector};


fn main() {
    let fragment = "<div onclick=\"someFunction()\">hello</div>";
    let parsed = Html::parse_fragment(fragment);
    let tree = parsed.tree.values();
    for item in tree {
        let val = item.as_element();
        match val {
            Option::None => {
                continue;
            }
            Option::Some(v) => {
                let x = v.attr("onclick").unwrap_or("ooops");
                
                println!("{x}");
            }
        }
        // println!("{val}");
    }

    // let fragment = Html::parse_fragment(r#"<input name="foo" value="bar">"#);
    // let selector = Selector::parse(r#"input[name="foo"]"#).unwrap();

    // let input = fragment.select(&selector).next().unwrap();
}
