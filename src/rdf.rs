use std::fs::File;
use std::io::{
    Read,
    Write
};

use rio_api::model::{
    NamedNode,
    Literal,
    Triple,
    Subject,
};
use rio_turtle::TurtleFormatter;
use rio_api::formatter::TriplesFormatter;

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

#[cfg(test)]
mod tests {
    use super::write;
    use super::MetaData;
    use std::io::stdout;
    use std::default::Default;
    use biblatex::EntryType;

    #[test]
    fn test_write() {
        let mut digest = Vec::with_capacity(64);
        digest.resize(64, 0x2a);
        let mut m = MetaData::new("foo", "bar", EntryType::Article, Vec::from(digest), None);
        m.set_subject("baz");
        m.set_mime_str("foo/bar");
        m.set_language("en-US");
        let v = stdout();
        let r = write(&m, v);
    }
}
