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
use crate::digest::RecordDigest;
use crate::digest::from_urn;

fn parse_digest(entry: &Entry) -> RecordDigest {
    let note = entry.get("note").unwrap();
    let note_s = String::from_chunks(note).unwrap();
    let mut digest_val = note_s.split(":");

    //let mut digest = Vec::new();
    let mut digest: RecordDigest = RecordDigest::Empty;

    match digest_val.next() {
        Some(v) => {
//            if v == "sha512" {
//                let digest_hex = digest_val.next().unwrap();
//                let mut digest_imported = hex::decode(digest_hex).unwrap();
//                digest.append(&mut digest_imported);
//                debug!("parsed digest {}", hex::encode(&digest));
//            }
                match from_urn(v) {
                    Ok(r) => {
                        digest = r;
                    },
                    Err(e) => {
                        debug!("note for entry {:?} is not a digest url", &entry);
                    },
                }
        },
        None => {},
    };
    
//    if digest.len() == 0 {
//        digest.resize(64, 0);
//    }

    digest
}

/// Read one or more metadata entries from the `bibtex` source.
///
/// Will return `ParseError` if any of the records are invalid.
///
/// # Arguments 
///
/// * `r` - reader implementation providing the source.
pub fn read_all(mut r: impl Read, digests: &Vec<RecordDigest>) -> Result<Vec<MetaData>, ParseError> {
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

        let mut use_digests: Vec<RecordDigest> = vec!();
        let digest = parse_digest(&e);
        match digest {
            RecordDigest::Empty => {
            },
            RecordDigest::Sha512(r) => {
                use_digests.push(RecordDigest::Sha512(r));
            },
            RecordDigest::Sha256(r) => {
                use_digests.push(RecordDigest::Sha256(r));
            },
            RecordDigest::MD5(r) => {
                use_digests.push(RecordDigest::MD5(r));
            },
            RecordDigest::SwarmHash(r) => {
                use_digests.push(RecordDigest::SwarmHash(r));
            },

//            RecordDigest::Sha512(r) => {
//                use_digests.push(digest);
//            },
//            RecordDigest::MD5(r) => {
//                use_digests.push(digest);
//            },
        }

        let title = e.title().unwrap();
        let title_s = String::from_chunks(title).unwrap();

        for dd in use_digests.into_iter() {
            let mut m = MetaData::new(title_s.as_str(), authors_s.as_str(), e.entry_type.clone(), dd, None);

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
    }
    Ok(rr)
}
