extern crate erased_serde;
extern crate serde_json;
extern crate serde_cbor;

use std::collections::BTreeMap as Map;
use std::io::{stdout, Read};

use erased_serde::{Serialize, Serializer, Deserializer};

#[test]
fn serialization() {
    // The values in this map are boxed trait objects. Ordinarily this would not
    // be possible with serde::Serializer because of object safety, but type
    // erasure makes it possible with erased_serde::Serializer.
    let mut formats: Map<&str, Box<Serializer>> = Map::new();
    formats.insert("json", Box::new(serde_json::ser::Serializer::new(stdout())));
    formats.insert("cbor", Box::new(serde_cbor::ser::Serializer::new(stdout())));

    // These are boxed trait objects as well. Same thing here - type erasure
    // makes this possible.
    let mut values: Map<&str, Box<Serialize>> = Map::new();
    values.insert("vec", Box::new(vec!["a", "b"]));
    values.insert("int", Box::new(65536));

    // Pick a Serializer out of the formats map.
    let format = formats.get_mut("json").unwrap();

    // Pick a Serialize out of the values map.
    let value = values.get("vec").unwrap();

    // This line prints `["a","b"]` to stdout.
    value.erased_serialize(format).unwrap();
}

#[test]
fn deserialization() {
    static JSON: &'static [u8] = br#"{"A": 65, "B": 66}"#;
    static CBOR: &'static [u8] = &[162, 97, 65, 24, 65, 97, 66, 24, 66];

    // The values in this map are boxed trait objects, which is not possible
    // with the normal serde::Deserializer because of object safety.
    let mut formats: Map<&str, Box<Deserializer>> = Map::new();
    formats.insert("json", Box::new(serde_json::de::Deserializer::new(JSON.bytes())));
    formats.insert("cbor", Box::new(serde_cbor::de::Deserializer::new(CBOR)));

    // Pick a Deserializer out of the formats map.
    let format = formats.get_mut("json").unwrap();

    let data: Map<String, usize> = erased_serde::deserialize(format).unwrap();

    println!("{}", data["A"] + data["B"]);
}
