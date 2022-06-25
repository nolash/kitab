use std::path;
use std::fmt;
use xattr;
use hex;

use crate::dc::DCMetaData;

pub type Digest = Vec<u8>;

pub type PublishDate = (u8, u8, u32);

pub type FileName = String;

pub type FilePath = String;

enum ResourceType {
    Unknown,
    Article,
    Whitepaper,
    Book,
    Report,
}

pub struct MetaData {
    dc: DCMetaData,
    typ: ResourceType,
    digest: Digest,
    local_name: Option<FileName>,
    comment: String,
    publish_date: PublishDate,
    retrieval_timestamp: u32,
}

impl MetaData {

    pub fn new(title: &str, author: &str, digest: Vec<u8>, filename: Option<FileName>) -> MetaData {
        let dc:DCMetaData = DCMetaData{
            title: String::from(title),
            author: String::from(author),
            subject: None,
        };
 
        MetaData{
                dc: dc,
                typ: ResourceType::Unknown,
                digest: vec!(),
                comment: String::new(),
                //local_name: filepath.to_str().unwrap().to_string(),
                local_name: filename,
                publish_date: (0, 0, 0),
                retrieval_timestamp: 0,
        }
    }

    pub fn title(&self) -> String {
        self.dc.title.clone()
    }

    pub fn author(&self) -> String {
        self.dc.author.clone()
    }

    pub fn fingerprint(&self) -> String {
        hex::encode(&self.digest)
    }

    pub fn from_xattr(filepath: &path::Path) -> MetaData {

        let mut title: String = String::new();
        let mut author: String = String::new();
        let mut subject: String = String::new();
        let filename: FileName; 

        let title_src = xattr::get(filepath, "user.dcterms:title").unwrap();
        match title_src {
            Some(v) => {
                let s = std::str::from_utf8(&v).unwrap();
                title.push_str(s);
            },
            None => {},
        }

        let author_src = xattr::get(filepath, "user.dcterms:creator").unwrap();
        match author_src {
            Some(v) => {
                let s = std::str::from_utf8(&v).unwrap();
                author.push_str(s);
            },
            None => {},
        }

        filename = filepath.file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();

        MetaData::new(title.as_str(), author.as_str(), vec!(), Some(filename))
    }
}

#[cfg(test)]
mod tests {
    use super::MetaData;
    use std::path;

    #[test]
    fn test_metadata_create() {
        let s = path::Path::new("testdata/bitcoin.pdf");
        let meta = MetaData::from_xattr(s);
        assert_eq!(meta.dc.title, "Bitcoin: A Peer-to-Peer Electronic Cash System");
        assert_eq!(meta.dc.author, "Satoshi Nakamoto");
    }
}
