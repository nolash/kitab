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


pub fn write_rdf(entry: &MetaData, w: impl Write) -> Result<usize, std::io::Error> {
    // TODO: parsers are apparently buggy, cannot decode dublin core rdf
    //let mut f = File::open("../dublincore/dublin_core_terms.rdf").unwrap();
    //let mut parser = NTriplesParser::from_reader(f);
    //let schema_graph: Graph = parser.decode().unwrap();

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
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::write_rdf;
    use super::MetaData;
    use std::io::stdout;

    #[test]
    fn test_write() {
        let m = MetaData::new("foo", "bar", vec!(0x2a), None);
        //let v =  Vec::default();
        let v = stdout();
        let r = write_rdf(&m, v);
    }
}
