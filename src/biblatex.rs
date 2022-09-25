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
    let note = match entry.get("note") {
        Some(v) => {
            v
        },
        None => {
            return RecordDigest::Empty;
        },
    };
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

        for v in digests {
            use_digests.push(v.clone());
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

#[cfg(test)]
mod tests {
    use super::read_all;
    use crate::digest;
    use env_logger;

    #[test]
    fn test_multi_digest() {
        let d_hex = "acbd18db4cc2f85cedef654fccc4a4d8";
        let d = digest::RecordDigest::MD5(hex::decode(d_hex).unwrap());
        let d_sha_hex = "f7fbba6e0636f890e56fbbf3283e524c6fa3204ae298382d624741d0dc6638326e282c41be5e4254d8820772c5518a2c5a8c0c7f7eda19594a7eb539453e1ed7";
        let d_sha = digest::from_vec(hex::decode(d_sha_hex).unwrap()).unwrap();

        let biblatex_src = "@article{
    foo,
    title={bar},
    author={Guybrush Threepwood},
}
";
        let digests = vec!(d, d_sha);
        let r = read_all(biblatex_src.as_bytes(), &digests).unwrap();

        assert_eq!(r.len(), 2);
    }
}
