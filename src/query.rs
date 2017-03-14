use queryst::parse;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct PageParams {
    pub size: i64,
    pub number: i64,
}

/// JSON-API Query parameters
#[derive(Debug, PartialEq)]
pub struct Query {
    pub _type: String,
    pub include: Option<Vec<String>>,
    pub fields: Option<HashMap<String, Vec<String>>>,
    pub page: Option<PageParams>,
}

/// JSON-API Query parameters
impl Query {
    ///
    /// Takes a query parameter string and returns a Query
    ///
    /// ```
    /// use jsonapi::query::Query;
    /// let query = Query::from_params("include=author&fields[articles]=title,\
    ///                                 body&fields[people]=name&page[number]=3&page[size]=1");
    /// match query.include {
    ///     None => assert!(false),
    ///     Some(include) => {
    ///         assert_eq!(include.len(), 1);
    ///         assert_eq!(include[0], "author");
    ///     }
    /// }
    ///
    /// ```
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

                let mut fields = HashMap::<String, Vec<String>>::new();

                o.find("fields").map(|x| if x.is_object() {
                    x.as_object().map(|obj| for (key, value) in obj.iter() {
                        let arr: Vec<String> = match value.as_str() {
                            Some(string) => string.split(",").map(|s| s.to_string()).collect(),
                            None => Vec::<String>::new(),
                        };
                        fields.insert(key.to_string(), arr);

                    });
                } else {
                    println!("No fields found in {:?}", x);
                });

                let page = PageParams {
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
                    _type: "none".into(),
                    include: include,
                    fields: Some(fields),
                    page: Some(page),
                }
            }
            Err(err) => {
                println!("Query: Can't parse : {:?}", err);
                Query {
                    _type: "none".into(),
                    include: None,
                    fields: None,
                    page: None,
                }
            }
        }
    }

    ///
    /// Builds a query parameter string from a Query
    ///
    /// ```
    /// use jsonapi::query::{Query, PageParams};
    /// let query = Query {
    ///   _type: "post".into(),
    ///   include: Some(vec!["author".into()]),
    ///   fields: None,
    ///   page: Some(PageParams {
    ///     size: 5,
    ///     number: 10,
    ///   }),
    /// };
    ///
    /// let query_string = query.to_params();
    /// assert_eq!(query_string, "include=author&page[size]=5&page[number]=10");
    ///
    /// ```
    pub fn to_params(&self) -> String {
        let mut params = Vec::<String>::new();

        match self.include {
            Some(ref include) => params.push(format!("include={}", include.join(","))),
            None => (),
        }

        // Examples from json-api.org,
        // fields[articles]=title,body,author&fields[people]=name
        // fields[articles]=title,body&fields[people]=name

        match self.fields {
            Some(ref fields) => {
                for (name, val) in fields.iter() {
                    params.push(format!("fields[{}]={}", name, val.join(",")));
                }
            }
            None => (),
        }

        match self.page {
            Some(ref page) => {
                params.push(page.to_params());
            }
            None => (),
        }

        params.join("&")
    }
}

impl PageParams {
    pub fn to_params(&self) -> String {
        format!("page[size]={}&page[number]={}", self.size, self.number)
    }
}
