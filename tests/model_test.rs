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
jsonapi_model!(Author; "authors"; has many books);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Book {
    id: String,
    title: String,
    first_chapter: Chapter,
    chapters: Vec<Chapter>
}
jsonapi_model!(Book; "books"; has one first_chapter; has many chapters);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Chapter {
    id: String,
    title: String,
    ordering: i32,
}
jsonapi_model!(Chapter; "chapters");

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
    let book_doc: DocumentData = serde_json::from_str(&json)
        .expect("Book DocumentData should be created from the book json");
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
    match &doc {
        JsonApiDocument::Error(_) => assert!(false),
        JsonApiDocument::Data(x) => {
            assert_eq!(x.data, Some(PrimaryData::Single(Box::new(res))));
        }
    }

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
fn from_jsonapi_document() {
    let json = ::read_json_file("data/author_tolkien.json");

    // TODO - is this the right thing that we want to test? Shold this cast into a JsonApiDocument
    // and detect if this was a data or an error?
    // Not sure that we want to immediately cast this into a "data" when we don't know if this isi
    // valid test file - it could be an error document for all we know... (that is equally valid)
    let author_doc: JsonApiDocument = serde_json::from_str(&json)
        .expect("Author DocumentData should be created from the author json");

    // This assumes that the fixture we're using is a "valid" document with data
    match author_doc {
        JsonApiDocument::Error(_) => assert!(false),
        JsonApiDocument::Data(doc) => {
            let author = Author::from_jsonapi_document(&doc)
                .expect("Author should be generated from the author_doc");

            let doc_again = author.to_jsonapi_document();
            assert!(doc_again.is_valid());
        }
    }
}
