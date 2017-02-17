use queryst::parse;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct PageQuery {
    pub page: i32,
    pub size: i32,
}

#[derive(Debug, PartialEq)]
pub struct Query {
    pub _type: String,
    pub include: Option<Vec<String>>,
    pub fields: Option<HashMap<String, Vec<String>>>,
    pub page: Option<PageQuery>,
}

/*
fn extract_array(obj: serde_json::value::Value, key: &str) -> Option<Vec<String>> {
    if let Some(thing) = obj.get(key) {
        if let Some(thing_str) = thing.as_str() {
            let arr: Vec<String> = thing_str.split(",").map(|s| s.to_string()).collect();
            println!("Including : {:?}", arr);
            Some(arr)
        } else {
            None
        }
    } else {
        None
    }
}
*/

impl Query {
    pub fn from_params(params: &str) -> Self {

        match parse(params) {
            Ok(o) => {
                println!("PARAMS : {:?}", o);
                // let include = extract_array(&o, &"include");

                let include = match o.find("include") {
                    None => None,
                    Some(inc) => {
                        match inc.as_str() {
                            None => None,
                            Some(include_str) => {
                                let arr: Vec<String> =
                                    include_str.split(",").map(|s| s.to_string()).collect();
                                Some(arr)
                            }
                        }
                    }
                };

                let fields = o.find("fields");
                println!("Fields : {:?}", fields);

                let page = o.find("page");
                println!("Page : {:?}", page);

                Query {
                    _type: format!("none"),
                    include: include,
                    fields: None,
                    page: None,
                }
            }
            Err(err) => {
                println!("Can't parse : {:?}", err);
                Query {
                    _type: format!("none"),
                    include: None,
                    fields: None,
                    page: None,
                }
            }
        }
    }
}
