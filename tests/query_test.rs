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

    match query.fields {
        None => assert!(false),
        Some(fields) => {
            assert_eq!(fields.contains_key("people"), true);
            assert_eq!(fields.contains_key("articles"), true);

            match fields.get("people") {
                None => assert!(false),
                Some(arr) => {
                    assert_eq!(arr.len(), 1);
                    assert_eq!(arr[0], "name");
                }
            }
            match fields.get("articles") {
                None => assert!(false),
                Some(arr) => {
                    assert_eq!(arr.len(), 2);
                    assert_eq!(arr[0], "title");
                    assert_eq!(arr[1], "body");
                }
            }
        }
    }

}
