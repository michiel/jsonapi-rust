use queryst::parse;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct PageQuery {
    pub size: i64,
    pub number: i64,
}

#[derive(Debug, PartialEq)]
pub struct Query {
    pub _type: String,
    pub include: Option<Vec<String>>,
    pub fields: Option<HashMap<String, Vec<String>>>,
    pub page: Option<PageQuery>,
}

impl Query {
    pub fn from_params(params: &str) -> Self {

        match parse(params) {
            Ok(o) => {
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

                // let fields = o.find("fields");

                let page = PageQuery {
                    number: match o.find_path(&["page", "number"]) {
                        None => 0,
                        Some(num) => {
                            if num.is_string() {
                                match num.as_str().map(str::parse::<i64>) {
                                    Some(y) => y.unwrap_or(0),
                                    None => 0,
                                }
                            } else {
                                0
                            }
                        }
                    },
                    size: match o.find_path(&["page", "size"]) {
                        None => 0,
                        Some(num) => {
                            if num.is_string() {
                                match num.as_str().map(str::parse::<i64>) {
                                    Some(y) => y.unwrap_or(0),
                                    None => 0,
                                }
                            } else {
                                0
                            }
                        }
                    },
                };

                Query {
                    _type: format!("none"),
                    include: include,
                    fields: None,
                    page: Some(page),
                }
            }
            Err(err) => {
                println!("Query: Can't parse : {:?}", err);
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
