use std::path;
use std::fmt;
use xattr;
use hex;
use mime::{
    Mime
};
use sha2::{
    Sha512,
    Digest,
};
use unic_langid_impl::LanguageIdentifier;
use biblatex::EntryType;
use std::str::FromStr;

use crate::dc::DCMetaData;

//pub type Digest = Vec<u8>;

pub type PublishDate = (u8, u8, u32);

pub type FileName = String;

pub type FilePath = String;

pub struct MetaData {
    dc: DCMetaData,
    digest: Vec<u8>,
    local_name: Option<FileName>,
    comment: String,
    publish_date: PublishDate,
    retrieval_timestamp: u32,
}

impl MetaData {
    pub fn new(title: &str, author: &str, typ: EntryType, digest: Vec<u8>, filename: Option<FileName>) -> MetaData {
        let dc = DCMetaData::new(title, author, typ);

        let sz = Sha512::output_size();
        if digest.len() != sz {
            panic!("wrong digest size, must be {}", sz);
        }

        MetaData{
                dc: dc,
                digest: digest,
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

    pub fn typ(&self) -> EntryType {
        self.dc.typ.clone()
    }

    pub fn set_subject(&mut self, v: &str) {
        self.dc.subject = Some(String::from(v));
    }

    pub fn subject(&self) -> Option<String> {
        return self.dc.subject.clone();
    }

    pub fn set_mime(&mut self, m: Mime) {
        self.dc.mime = Some(m);
    }

    pub fn set_mime_str(&mut self, s: &str) {
        match Mime::from_str(s) {
            Ok(v) => {
                self.set_mime(v);
            },
            Err(e) => {
                panic!("invalid mime");
            },
        };
    }

    pub fn mime(&self) -> Option<Mime> {
        self.dc.mime.clone()
    }

    pub fn set_language(&mut self, s: &str) {
        let v = s.parse().unwrap();
        self.dc.language = Some(v);
    }

    pub fn language(&self) -> Option<LanguageIdentifier> {
        self.dc.language.clone()
    }

    pub fn fingerprint(&self) -> String {
        hex::encode(&self.digest)
    }

    pub fn from_xattr(filepath: &path::Path) -> MetaData {

        let mut title: String = String::new();
        let mut author: String = String::new();
        let mut typ: EntryType = EntryType::Unknown(String::new());
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

        let typ_src = xattr::get(filepath, "user.dcterms:type").unwrap();
        match typ_src {
            Some(v) => {
                let s = std::str::from_utf8(&v).unwrap();
                typ = EntryType::new(s);
            },
            None => {},
        }

        let mut metadata = MetaData::new(title.as_str(), author.as_str(), typ, vec!(), Some(filename));

        match xattr::get(filepath, "user.dcterms:subject") {
            Ok(v) => {
                match v {
                    Some(v) => {
                        let s = std::str::from_utf8(&v).unwrap();
                        metadata.set_subject(s);
                    },
                    None => {},
                }
            },
            _ => {},
        };

        match xattr::get(filepath, "user.dcterms:MediaType") {
            Ok(v) => {
                match v {
                    Some(v) => {
                        let s = std::str::from_utf8(&v).unwrap();
                        metadata.set_mime_str(s);
                    },
                    None => {},
                }
            },
            _ => {},
        }

        metadata
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
