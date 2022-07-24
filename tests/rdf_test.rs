use std::io::{
    Write,
    BufWriter,
};

use biblatex::EntryType;

use kitab::rdf::write as rdf_write;
use kitab::meta::MetaData;


#[test]
fn test_rdf_dump() {
    let v = Vec::new();
    let w = BufWriter::new(v);
    let mut digest: Vec<u8> = Vec::new();
    digest.resize(64, 0);
    let metadata = MetaData::new("foo", "Bar Baz", EntryType::Article, digest, None);
    let r = rdf_write(&metadata, w);
}
