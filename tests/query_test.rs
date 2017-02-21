extern crate jsonapi;
extern crate serde_json;

use jsonapi::query::*;

#[test]
fn can_parse() {
    let query = Query::from_params("include=author&fields[articles]=title,\
                                    body&fields[people]=name&page[number]=3&page[size]=1");

    match query.include {
        None => assert!(false),
        Some(include) => {
            assert_eq!(include.len(), 1);
            assert_eq!(include[0], "author");
        }
    }

    match query.page {
        None => assert!(false),
        Some(page) => {
            assert_eq!(page.size, 1);
            assert_eq!(page.number, 3);
        }
    }

}
