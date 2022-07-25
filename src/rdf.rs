use std::fs::File;
use std::io::{
    Read,
    Write
};
use std::str::FromStr;
use std::io::{
    BufReader,
};

use rio_turtle::{
    TurtleParser,
    TurtleError,
    TurtleFormatter,
};
use rio_api::parser::TriplesParser;
use rio_api::formatter::TriplesFormatter;
use rio_api::model::{
    NamedNode,
    Literal,
    Triple,
    Subject,
};
use urn::{
    Urn,
    Error as UrnError,
};

use log::{
    debug,
    info,
    error,
};

use crate::meta::MetaData;
use crate::error::ParseError;
use crate::dc::{
    DC_IRI_TITLE,
    DC_IRI_CREATOR,
    DC_IRI_SUBJECT,
    DC_IRI_LANGUAGE,
    DC_IRI_TYPE,
    DC_IRI_MEDIATYPE,
};

#[derive(Debug)]
pub enum RdfError {
    UrnError(UrnError),
    HashMismatchError,
}

pub fn write(entry: &MetaData, w: impl Write) -> Result<usize, std::io::Error> {
    let mut tfmt = TurtleFormatter::new(w);
    
    let urn_str = format!("URN:sha512:{}", entry.fingerprint());
    let urn = Subject::NamedNode(
        NamedNode{
            iri: urn_str.as_str(),
        },
    );

    tfmt.format(&Triple{
        subject: urn,
        predicate: NamedNode { iri: DC_IRI_TITLE }.into(),
        object: Literal::Simple { value: entry.title().as_str() }.into(),
    });
    tfmt.format(&Triple{
        subject: urn,
        predicate: NamedNode { iri: DC_IRI_CREATOR }.into(),
        object: Literal::Simple { value: entry.author().as_str() }.into(),
    });
    let typ = entry.typ().to_string();
    tfmt.format(&Triple{
        subject: urn,
        predicate: NamedNode { iri: DC_IRI_TYPE }.into(),
        object: Literal::Simple { value: typ.as_str() }.into(),
    });
    match entry.subject() {
        Some(v) => {
            tfmt.format(&Triple{
                subject: urn,
                predicate: NamedNode { iri: DC_IRI_SUBJECT }.into(),
                object: Literal::Simple { value: v.as_str() }.into(),
            });
        },
        _ => (),
    };

    match entry.mime() {
        Some(v) => {
            let m: String = v.to_string();
            tfmt.format(&Triple{
                subject: urn,
                predicate: NamedNode { iri: DC_IRI_MEDIATYPE }.into(),
                object: Literal::Simple { value: m.as_str() }.into(),
            });
        },
        _ => (),
    };

    match entry.language() {
        Some(v) => {
            let m: String = v.to_string();
            tfmt.format(&Triple{
                subject: urn,
                predicate: NamedNode { iri: DC_IRI_LANGUAGE }.into(),
                object: Literal::Simple { value: m.as_str() }.into(),
            });
        },
        _ => (),
    };

    tfmt.finish();
    Ok(0)
}


fn handle_parse_match(metadata: &mut MetaData, triple: Triple) -> Result<(), RdfError> {
    let subject_iri = triple.subject.to_string();
    let l = subject_iri.len()-1;
    let subject = &subject_iri[1..l];
    let subject_urn = Urn::from_str(subject).unwrap();
    if subject_urn.nid() != "sha512" {
        return Err(RdfError::UrnError(UrnError::InvalidNid));
    }

    let v = subject_urn.nss();
    let b = hex::decode(&v).unwrap();
    if metadata.fingerprint().len() == 0 {
        debug!("setting fingerprint {}", v);
        metadata.set_fingerprint(b);
    } else if metadata.fingerprint() != v {
        return Err(RdfError::HashMismatchError);
    }

    let field = triple.predicate.iri;
    match field {
        DC_IRI_TITLE => {
            let title = triple.object.to_string().replace("\"", "");
            metadata.set_title(title.as_str());
            debug!("found title: {}", title);
        },
        DC_IRI_CREATOR => {
            let author = triple.object.to_string().replace("\"", "");
            metadata.set_author(author.as_str());
            debug!("found author: {}", author);
        },
        DC_IRI_SUBJECT => {
            let mut subject = triple.object.to_string().replace("\"", "");
            metadata.set_subject(subject.as_str());
            debug!("found subject: {}", subject);
        },
        DC_IRI_LANGUAGE => {
            let mut lang = triple.object.to_string().replace("\"", "");
            metadata.set_language(lang.as_str());
            debug!("found language: {}", lang);
        },
        DC_IRI_TYPE => {
            let mut typ = triple.object.to_string().replace("\"", "");
            metadata.set_typ(typ.as_str());
            debug!("found entry type: {}", typ);
        },
        DC_IRI_MEDIATYPE => {
            let mut mime_type = triple.object.to_string().replace("\"", "");
            metadata.set_mime_str(mime_type.as_str());
            debug!("found mime type: {}", mime_type);
        },
        _ => {
            debug!("skipping unknown predicate: {}", field);
        },
    };
    Ok(())
}

pub fn read_all(r: impl Read) -> Result<Vec<MetaData>, ParseError> {
    let mut rr: Vec<MetaData> = vec!();
    let bf = BufReader::new(r);
    let mut tp = TurtleParser::new(bf, None);
    rr.push(MetaData::empty());
    let mut i: usize = 0;
    let r: Result<_, TurtleError> = tp.parse_all(&mut |r| {
        match r {
            Triple{subject, predicate, object } => {
                match handle_parse_match(&mut rr[i], r) {
                    Err(HashMismatchError) => {
                        rr.push(MetaData::empty());
                        i += 1;
                        match handle_parse_match(&mut rr[i], r) {
                            Err(e) => {
                                error!("{:?}", e);
                            },
                            _ => {},
                        };
                    },
                    _ => {},
                };
            },
        }
        Ok(())
    });
    // TODO: should check validity of all records
    if rr[0].fingerprint() == "" {
        return Err(ParseError);
    }
    Ok(rr)
}
pub fn read(r: impl Read) -> MetaData {
    let mut rr: Vec<MetaData> = vec!();
    let mut metadata = MetaData::empty();
    let bf = BufReader::new(r);
    let mut tp = TurtleParser::new(bf, None);
    let r: Result<_, TurtleError> = tp.parse_all(&mut |r| {
        match r {
            Triple{subject, predicate, object } => {
                handle_parse_match(&mut metadata, r);
            },
            _ => {},
        }
        Ok(())
    });
    metadata
}

#[cfg(test)]
mod tests {
    use super::{
        write,
        read,
    };
    use super::MetaData;
    use std::io::stdout;
    use std::fs::File;
    use std::default::Default;
    use biblatex::EntryType;
    use env_logger;

    #[test]
    fn test_turtle_write() {
        let mut digest = Vec::with_capacity(64);
        digest.resize(64, 0x2a);
        let mut m = MetaData::new("foo", "bar", EntryType::Article, Vec::from(digest), None);
        m.set_subject("baz");
        m.set_mime_str("foo/bar");
        m.set_language("nb-NO");
        //let v = stdout();
        let mut v: Vec<u8> = vec!();
        let r = write(&m, v);
    }

    #[test]
    fn test_turtle_read() {
        let f = File::open("testdata/meta.ttl").unwrap();
        read(&f);
    }
}
