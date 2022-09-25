use std::io::{
    Write,
    BufWriter,
};

use biblatex::EntryType;

use kitab::rdf::write as rdf_write;
use kitab::meta::MetaData;

use kitab::digest;


#[test]
fn test_rdf_dump() {
    let v = Vec::new();
    let w = BufWriter::new(v);
    let mut digest: Vec<u8> = Vec::new();
    digest.resize(64, 0);
    let digest_sha = digest::from_vec(Vec::from(digest)).unwrap();
    let metadata = MetaData::new("foo", "Bar Baz", EntryType::Article, digest_sha, None);
    let r = rdf_write(&metadata, w);
}
