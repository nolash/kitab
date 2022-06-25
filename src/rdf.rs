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
};

use crate::meta::MetaData;



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
        predicate: NamedNode { iri: "https://purl.org/dc/terms/title" }.into(),
        object: Literal::Simple { value: entry.title().as_str() }.into(),
    });
    tfmt.format(&Triple{
        subject: urn,
        predicate: NamedNode { iri: "https://purl.org/dc/terms/creator" }.into(),
        object: Literal::Simple { value: entry.author().as_str() }.into(),
    });
    let typ = entry.typ().to_string();
    tfmt.format(&Triple{
        subject: urn,
        predicate: NamedNode { iri: "https://purl.org/dc/terms/type" }.into(),
        object: Literal::Simple { value: typ.as_str() }.into(),
    });
    match entry.subject() {
        Some(v) => {
            tfmt.format(&Triple{
                subject: urn,
                predicate: NamedNode { iri: "https://purl.org/dc/terms/subject" }.into(),
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
                predicate: NamedNode { iri: "https://purl.org/dc/terms/MediaType" }.into(),
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
                predicate: NamedNode { iri: "https://purl.org/dc/terms/language" }.into(),
                object: Literal::Simple { value: m.as_str() }.into(),
            });
        },
        _ => (),
    };

    tfmt.finish();
    Ok(0)
}


pub fn handle_parse_match(metadata: &mut MetaData, triple: Triple) -> Result<(), UrnError> {
    let subject_iri = triple.subject.to_string();
    let l = subject_iri.len()-1;
    let subject = &subject_iri[1..l];
    let subject_urn = Urn::from_str(subject).unwrap();
    if subject_urn.nid() != "sha512" {
        return Err(UrnError::InvalidNid);
    }

    if metadata.fingerprint().len() == 0 {
        let v = subject_urn.nss();
        let b = hex::decode(&v).unwrap();
        info!("setting fingerprint {}", v);
        metadata.set_fingerprint(b);
    }

    let field = triple.predicate.iri;
    match field {
        "https://purl.org/dc/terms/title" => {
            let title = triple.object.to_string().replace("\"", "");
            metadata.set_title(title.as_str());
            info!("found title: {}", title);
        },
        "https://purl.org/dc/terms/creator" => {
            let author = triple.object.to_string().replace("\"", "");
            metadata.set_author(author.as_str());
            info!("found author: {}", author);
        },
        "https://purl.org/dc/terms/subject" => {
            let mut subject = triple.object.to_string().replace("\"", "");
            metadata.set_subject(subject.as_str());
            info!("found subject: {}", subject);
        },
        "https://purl.org/dc/terms/language" => {
            let mut lang = triple.object.to_string().replace("\"", "");
            metadata.set_language(lang.as_str());
            info!("found language: {}", lang);
        },
        "https://purl.org/dc/terms/type" => {
            let mut typ = triple.object.to_string().replace("\"", "");
            metadata.set_typ(typ.as_str());
            info!("found entry type: {}", typ);
        },
        "https://purl.org/dc/terms/MediaType" => {
            let mut mime_type = triple.object.to_string();
            let l = mime_type.len()-1;
            metadata.set_mime_str(&mime_type[1..l]);
            info!("found mime type: {}", mime_type);
        },
        _ => {
            debug!("skipping unknown predicate: {}", field);
        },
    };
    Ok(())
}

pub fn read(r: impl Read) {
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
        m.set_language("en-US");
        let v = stdout();
        let r = write(&m, v);
    }

    #[test]
    fn test_turtle_read() {
        env_logger::init();

        let f = File::open("testdata/meta.ttl").unwrap();
        read(&f);
    }
}
