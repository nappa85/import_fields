
use import_fields::import_fields;

use serde::{Serialize, Deserialize};

mod example;
use example::des;

#[test]
fn import_all() {
    #[import_fields("tests/example.rs")]
    #[derive(Serialize, Deserialize, Debug)]
    struct Foo {
    }

    let test: Foo = dbg!(serde_json::from_str(r#"{"foo": "hello", "i":"2"}"#)).unwrap();
    assert_eq!(test.bar, "hello");
    assert_eq!(test.i, Some(2));
}

#[test]
fn import_one() {
    #[import_fields("tests/example.rs::Test1")]
    #[derive(Serialize, Deserialize, Debug)]
    struct Foo {
    }

    let test: Foo = dbg!(serde_json::from_str(r#"{"foo": "hello", "i":"2"}"#)).unwrap();
    assert_eq!(test.bar, "hello");
}


#[test]
fn import_many() {
    #[import_fields("tests/example.rs::Test1", "tests/example.rs::Test2")]
    #[derive(Serialize, Deserialize, Debug)]
    struct Foo {
    }

    let test: Foo = dbg!(serde_json::from_str(r#"{"foo": "hello", "i":"2"}"#)).unwrap();
    assert_eq!(test.bar, "hello");
    assert_eq!(test.i, Some(2));
}


#[test]
fn import_multi() {
    #[import_fields("tests/example.rs::Test1")]
    #[import_fields("tests/example.rs::Test2")]
    #[derive(Serialize, Deserialize, Debug)]
    struct Foo {
    }

    let test: Foo = dbg!(serde_json::from_str(r#"{"foo": "hello", "i":"2"}"#)).unwrap();
    assert_eq!(test.bar, "hello");
    assert_eq!(test.i, Some(2));
}
