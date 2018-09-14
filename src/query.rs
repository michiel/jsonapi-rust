use queryst::parse;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PageParams {
    pub size: i64,
    pub number: i64,
}

/// JSON-API Query parameters
#[derive(Debug, PartialEq, Default)]
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
                let include = match o.pointer("/include") {
                    None => None,
                    Some(inc) => {
                        match inc.as_str() {
                            None => None,
                            Some(include_str) => {
                                let arr: Vec<String> =
                                    include_str.split(',').map(|s| s.to_string()).collect();
                                Some(arr)
                            }
                        }
                    }
                };

                let mut fields = HashMap::<String, Vec<String>>::new();

                if let Some(x) = o.pointer("/fields") {
                    if x.is_object() {
                        if let Some(obj) = x.as_object() {
                            for (key, value) in obj.iter() {
                                let arr: Vec<String> = match value.as_str() {
                                    Some(string) => string.split(',').map(|s| s.to_string()).collect(),
                                    None => Vec::<String>::new(),
                                };
                                fields.insert(key.to_string(), arr);

                            }
                        }
                    } else {
                        error!("Query::from_params : No fields found in {:?}", x);
                    }
                }

                let page = PageParams {
                    number: match o.pointer("/page/number") {
                        None => {
                            warn!("Query::from_params : No page/number found in {:?}, setting \
                                   default 0",
                                  o);
                            0
                        }
                        Some(num) => {
                            if num.is_string() {
                                match num.as_str().map(str::parse::<i64>) {
                                    Some(y) => y.unwrap_or(0),
                                    None => {
                                        warn!("Query::from_params : page/number found in {:?}, \
                                               not able not able to parse it - setting default 0",
                                              o);
                                        0
                                    }
                                }
                            } else {
                                warn!("Query::from_params : page/number found in {:?}, but it is \
                                       not an expected type - setting default 0",
                                      o);
                                0
                            }
                        }
                    },
                    size: match o.pointer("/page/size") {
                        None => {
                            warn!("Query::from_params : No page/size found in {:?}, setting \
                                   default 0",
                                  o);
                            0
                        }
                        Some(num) => {
                            if num.is_string() {
                                match num.as_str().map(str::parse::<i64>) {
                                    Some(y) => y.unwrap_or(0),
                                    None => {
                                        warn!("Query::from_params : page/size found in {:?}, \
                                               not able not able to parse it - setting default 0",
                                              o);
                                        0
                                    }
                                }
                            } else {
                                warn!("Query::from_params : page/size found in {:?}, but it is \
                                       not an expected type - setting default 0",
                                      o);
                                0
                            }
                        }
                    },
                };

                Query {
                    _type: "none".into(),
                    include,
                    fields: Some(fields),
                    page: Some(page),
                }
            }
            Err(err) => {
                error!("Query::from_params : Can't parse : {:?}", err);
                Query {
                    _type: "none".into(),
                    ..Default::default()
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

        if let Some(ref include) = self.include {
            params.push(format!("include={}", include.join(",")));
        }

        // Examples from json-api.org,
        // fields[articles]=title,body,author&fields[people]=name
        // fields[articles]=title,body&fields[people]=name

        if let Some(ref fields) = self.fields {
            for (name, val) in fields.iter() {
                params.push(format!("fields[{}]={}", name, val.join(",")));
            }
        }

        if let Some(ref page) = self.page {
            params.push(page.to_params());
        }

        params.join("&")
    }
}

impl PageParams {
    pub fn to_params(&self) -> String {
        format!("page[size]={}&page[number]={}", self.size, self.number)
    }
}
