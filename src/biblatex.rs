use std::io::{
    Read,
};
use std::str;

use log::{
    debug,
    error,
};
use biblatex::{
    Bibliography,
    Type,
    Entry as Entry,
};

use crate::meta::MetaData;
use crate::error::ParseError;

fn parse_digest(entry: &Entry) -> Vec<u8> {
    let note = entry.get("note").unwrap();
    let note_s = String::from_chunks(note).unwrap();
    let mut digest_val = note_s.split(":");

    let mut digest = Vec::new();

    match digest_val.next() {
        Some(v) => {
            if v == "sha512" {
                let digest_hex = digest_val.next().unwrap();
                let mut digest_imported = hex::decode(digest_hex).unwrap();
                digest.append(&mut digest_imported);
                debug!("parsed digest {}", hex::encode(&digest));
            }
        },
        None => {},
    };
    
    if digest.len() == 0 {
        digest.resize(64, 0);
    }

    digest
}

pub fn read_all(mut r: impl Read) -> Result<Vec<MetaData>, ParseError> {
    let mut s = String::new();
    let c = r.read_to_string(&mut s);
    let bib = match Bibliography::parse(&s) {
        Ok(v) => {
            v
        },
        Err(e) => {
            error!("parse error for biblatex");
            return Err(ParseError);
        },
    };

    let mut rr: Vec<MetaData> = vec!();

    for e in bib.iter() {
        let authors = e.author()
            .unwrap()
            .into_iter()
            .map(|v| {
            format!("{} {}", v.given_name, v.name)
        });
        let authors_s = authors.fold(String::new(), |x, y| {
            if x.len() == 0 {
                return y
            }
            format!("{}, {}", x, y)
        });
        let digest = parse_digest(&e);

        let title = e.title().unwrap();
        let title_s = String::from_chunks(title).unwrap();

        let mut m = MetaData::new(title_s.as_str(), authors_s.as_str(), e.entry_type.clone(), digest, None);

        match e.keywords() {
            Ok(v) => {
                let s = String::from_chunks(v).unwrap();
                m.set_subject(s.as_str());
            },
            _ => {},
        };

        match e.language() {
            Ok(v) => {
                m.set_language(v.as_str());
            },
            _ => {},
        }

        debug!("read metadata {:?}", &m);
        rr.push(m);
    }
    Ok(rr)
}
