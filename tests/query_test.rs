extern crate jsonapi;
extern crate serde_json;

use jsonapi::query::*;

#[test]
fn can_parse() {
    let query = Query::from_params("include=author&fields[articles]=title,\
                                    body&fields[people]=name&page[number]=3&page[size]=1");
    println!("Received : {:?}", query);
    assert_eq!(1, 1);
}
