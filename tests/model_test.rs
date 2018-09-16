#[macro_use]
extern crate jsonapi;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use jsonapi::model::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Dog {
    id: String,
    name: String,
    age: i32,
    main_flea: Flea,
    fleas: Vec<Flea>,
}
jsonapi_model!(Dog; "dog"; has one main_flea; has many fleas);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Flea {
    id: String,
    name: String,
}
jsonapi_model!(Flea; "flea");

#[test]
fn to_jsonapi_document_and_back() {
    let dog = Dog {
        id: "1".into(),
        name: "fido".into(),
        age: 2,
        main_flea: Flea {
            id: "1".into(),
            name: "general flea".into(),
        },
        fleas: vec![
            Flea {
                id: "2".into(),
                name: "rick".into(),
            },
            Flea {
                id: "3".into(),
                name: "morty".into(),
            },
        ],
    };
    let doc = dog.to_jsonapi_document();
    let json = serde_json::to_string(&doc).unwrap();
    println!("JSON IS:");
    let dog_doc: JsonApiDocument = serde_json::from_str(&json).expect(
        "Dog JsonApiDocument should be created from the dog json",
    );
    let dog_again =
        Dog::from_jsonapi_document(&dog_doc).expect("Dog should be generated from the dog_doc");

    assert_eq!(dog, dog_again);
}

#[test]
fn numeric_id() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct NumericFlea {
        id: i32,
        name: String,
    }
    jsonapi_model!(NumericFlea; "numeric_flea");

    let flea = NumericFlea {
        id: 2,
        name: "rick".into(),
    };
    let (res, _) = flea.to_jsonapi_resource();
    assert_eq!(res.id, "2".to_string());
    let doc = flea.to_jsonapi_document();
    assert!(doc.is_valid());
    assert_eq!(doc.data, Some(PrimaryData::Single(Box::new(res))));
    let json = serde_json::to_string(&doc).unwrap();
    let _num_doc: JsonApiDocument = serde_json::from_str(&json).expect(
        "NumericFlea JsonApiDocument should be created from the flea json",
    );
}

#[test]
fn test_vec_to_jsonapi_document() {
    let fleas = vec![
        Flea {
            id: "2".into(),
            name: "rick".into(),
        },
        Flea {
            id: "3".into(),
            name: "morty".into(),
        },
    ];
    let doc = vec_to_jsonapi_document(fleas);
    assert!(doc.is_valid());
}

#[test]
fn from_jsonapi_document_and_back() {}
