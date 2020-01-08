#[macro_use]
extern crate jsonapi;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use jsonapi::array::JsonApiArray;
use jsonapi::model::*;

mod helper;
use helper::read_json_file;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Author {
    id: String,
    name: String,
    books: Vec<Book>,
}
jsonapi_model!(Author; "author"; has many books);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Book {
    id: String,
    title: String,
    first_chapter: Chapter,
    chapters: Vec<Chapter>
}
jsonapi_model!(Book; "book"; has one first_chapter; has many chapters);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Chapter {
    id: String,
    title: String,
    ordering: i32,
}
jsonapi_model!(Chapter; "chapter");

#[test]
fn to_jsonapi_document_and_back() {
    let book = Book {
        id: "1".into(),
        title: "The Fellowship of the Ring".into(),
        first_chapter: Chapter { id: "1".into(), title: "A Long-expected Party".into(), ordering: 1 },
        chapters: vec![
            Chapter { id: "1".into(), title: "A Long-expected Party".into(), ordering: 1 },
            Chapter { id: "2".into(), title: "The Shadow of the Past".into(), ordering: 2 },
            Chapter { id: "3".into(), title: "Three is Company".into(), ordering: 3 }
        ],
    };

    let doc = book.to_jsonapi_document();
    let json = serde_json::to_string(&doc).unwrap();
    let book_doc: JsonApiDocument = serde_json::from_str(&json)
        .expect("Book JsonApiDocument should be created from the book json");
    let book_again = Book::from_jsonapi_document(&book_doc)
        .expect("Book should be generated from the book_doc");

    assert_eq!(book, book_again);
}

#[test]
fn numeric_id() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct NumericChapter {
        id: i32,
        title: String,
    }
    jsonapi_model!(NumericChapter; "numeric_chapter");

    let chapter = NumericChapter {
        id: 24,
        title: "The Riders of Rohan".into(),
    };

    let (res, _) = chapter.to_jsonapi_resource();
    assert_eq!(res.id, "24".to_string());

    let doc = chapter.to_jsonapi_document();
    assert!(doc.is_valid());
    assert_eq!(doc.data, Some(PrimaryData::Single(Box::new(res))));

    let json = serde_json::to_string(&doc).unwrap();
    let _num_doc: JsonApiDocument = serde_json::from_str(&json)
        .expect("NumericChapter JsonApiDocument should be created from the chapter json");
}

#[test]
fn test_vec_to_jsonapi_document() {
    let chapters = vec![
        Chapter {
            id: "45".into(),
            title: "The Passing of the Grey Company".into(),
            ordering: 2,
        },
        Chapter {
            id: "46".into(),
            title: "The Muster of Rohan".into(),
            ordering: 3,
        },
    ];

    let doc = vec_to_jsonapi_document(chapters);
    assert!(doc.is_valid());
}

#[test]
fn from_jsonapi_document_and_back() {
    let json = ::read_json_file("data/author_tolkien.json");

    let author_doc: JsonApiDocument = serde_json::from_str(&json)
        .expect("Author JsonApiDocument should be created from the author json");
    let author = Author::from_jsonapi_document(&author_doc)
        .expect("Author should be generated from the author_doc");

    let doc_again = author.to_jsonapi_document();

    assert_eq!(author_doc, doc_again);
}
