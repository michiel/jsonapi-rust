use queryst::parse;
use std::collections::HashMap;
use serde_json; //::value::Value;

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

fn extract_array(obj: serde_json::value::Value, key: &str) -> Option<Vec<String>> {
    if let Some(thingo) = obj.get(key) {
        if let Some(thingo_str) = thingo.as_str() {
            let arr: Vec<String> = thingo_str.split(",").map(|s| s.to_string()).collect();
            println!("Including : {:?}", arr);
            Some(arr)
        } else {
            None
        }
    } else {
        None
    }
}

impl Query {
    pub fn from_params(params: &str) -> Self {

        match parse(params) {
            Ok(o) => {
                println!("PARAMS : {:?}", o);
                /*
                if let Some(include) = o.find("include") {
                    if let Some(include_str) = include.as_str() {
                        let arr: Vec<String> =
                            include_str.split(",").map(|s| s.to_string()).collect();
                        println!("Including : {:?}", arr);
                    } else {
                    }

                } else {
                }
                */

                let include = extract_array(o, "include");

                let fields = o.find("fields");
                println!("Fields : {:?}", fields);

                let page = o.find("page");
                println!("Page : {:?}", page);

                Query {
                    _type: format!("none"),
                    include: None,
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
