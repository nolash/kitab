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
use std::fs::{
    File,
    metadata,
};
use std::path::Path;
use std::io::{
    Read,
    BufRead,
    BufReader,
};
use unic_langid_impl::LanguageIdentifier;
use biblatex::EntryType;
use std::str::FromStr;
use std::os::linux::fs::MetadataExt;

#[cfg(feature = "magic")]
use tree_magic;

use crate::dc::{
    DCMetaData,
    DC_XATTR_TITLE,
    DC_XATTR_CREATOR,
    DC_XATTR_SUBJECT,
    DC_XATTR_LANGUAGE,
    DC_XATTR_TYPE,
    DC_XATTR_MEDIATYPE,
};

use log::{
    debug,
};

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

pub fn check_xattr() {

}

pub fn digest_from_path(filepath: &path::Path) -> Vec<u8> {
        let mut h = Sha512::new();
        let st = metadata(filepath).unwrap();
        let bs: u64 = st.st_blksize();
        let sz: u64 = st.st_size();
        let mut b: Vec<u8> = vec!(0; bs as usize);
        let mut f = File::open(filepath).unwrap();
        let mut i: usize = 0;
        while i < sz as usize {
            let c = f.read(&mut b).unwrap();
            h.update(&b[..c]);
            i += c;
        }
        h.finalize().to_vec()
    }
impl MetaData {
    pub fn new(title: &str, author: &str, typ: EntryType, digest: Vec<u8>, filename: Option<FileName>) -> MetaData {
        let dc = DCMetaData::new(title, author, typ);

        let mut m = MetaData{
                dc: dc,
                digest: vec!(),
                comment: String::new(),
                local_name: filename,
                publish_date: (0, 0, 0),
                retrieval_timestamp: 0,
        };

        m.set_fingerprint(digest);
        m
    }

    pub fn empty() -> MetaData {
        let dc = DCMetaData::new("", "", EntryType::Unknown(String::new()));
        MetaData{
                dc: dc,
                digest: vec!(),
                comment: String::new(),
                //local_name: filepath.to_str().unwrap().to_string(),
                local_name: None,
                publish_date: (0, 0, 0),
                retrieval_timestamp: 0,
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.dc.title = String::from(title);
    }

    pub fn set_author(&mut self, author: &str) {
        self.dc.author = String::from(author);
    }

    pub fn set_fingerprint(&mut self, fingerprint: Vec<u8>) {
        let sz = Sha512::output_size();
        if fingerprint.len() != sz {
            panic!("wrong digest size, must be {}", sz);
        }
        self.digest = fingerprint;
    }

    pub fn title(&self) -> String {
        self.dc.title.clone()
    }

    pub fn author(&self) -> String {
        self.dc.author.clone()
    }

    pub fn set_typ(&mut self, typ: &str) {
        self.dc.typ = EntryType::from_str(typ).unwrap();
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

        let digest = digest_from_path(filepath);

        filename = filepath.file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();

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


        let typ_src = xattr::get(filepath, "user.dcterms:type").unwrap();
        match typ_src {
            Some(v) => {
                let s = std::str::from_utf8(&v).unwrap();
                typ = EntryType::new(s);
            },
            None => {},
        }

        let mut metadata = MetaData::new(title.as_str(), author.as_str(), typ, digest, Some(filename));

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

        match xattr::get(filepath, "user.dcterms:language") {
            Ok(v) => {
                match v {
                    Some(v) => {
                        let s = std::str::from_utf8(&v).unwrap();
                        metadata.set_language(s);
                    },
                    None => {},
                }
            },
            _ => {},
        }

        #[cfg(feature = "magic")]
        metadata.set_mime_magic(filepath);

        metadata
    }


    pub fn to_xattr(&self, filepath: &path::Path) -> Result<(), std::io::Error> {
        let filename = filepath.file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();

        xattr::set(filepath, DC_XATTR_TITLE, self.dc.title.as_bytes());
        xattr::set(filepath, DC_XATTR_CREATOR, self.dc.author.as_bytes());
        xattr::set(filepath, DC_XATTR_TYPE, self.dc.typ.to_string().as_bytes());

        match &self.dc.language {
            Some(v) => {
                xattr::set(filepath, DC_XATTR_LANGUAGE, v.to_string().as_bytes());
            },
            _ => {},
        };

        match &self.dc.mime {
            Some(v) => {
                xattr::set(filepath, DC_XATTR_MEDIATYPE, v.to_string().as_bytes());
            },
            _ => {},
        };

        match &self.dc.subject {
            Some(v) => {
                xattr::set(filepath, DC_XATTR_SUBJECT, v.as_bytes());
            },
            _ => {},
        };

        Ok(())
    }

    fn process_predicate(&mut self, predicate: &str, object: &str) -> bool {
        match predicate.to_lowercase().as_str() {
            "title" => {
                self.set_title(object);
                debug!("found title: {}", object);
            },
            "author" => {
                self.set_author(object);
                debug!("found author: {}", object);
            },
            "subject" => {
                self.set_subject(object);
                debug!("found subject: {}", object);
            },
            "typ" => {
                self.set_typ(object);
                debug!("found typ: {}", object);
            },
            "language" => {
                self.set_language(object);
                debug!("found language: {}", object);
            },
            "mime" => {
                self.set_mime_str(object);
                debug!("found mime: {}", object);
            },
            _ => {
                return false;
            },
        }
        true
    }

    fn process_line(&mut self, s: &str) {
        match s.split_once(":") {
            Some((predicate, object_raw)) => {
                let object = object_raw.trim();
                self.process_predicate(predicate, object);
            },
            None => {
            },
        }
    }

    #[cfg(feature = "magic")]
    pub fn set_mime_magic(&mut self, path: &path::Path) {
        if self.mime() == None {
            let mime = tree_magic::from_filepath(path);
            self.set_mime_str(&mime);
            debug!("magic set mime {}", mime);
        }
    }

    pub fn from_path(p: &path::Path) -> Result<MetaData, std::io::Error> {
        let f = File::open(&p).unwrap();
        debug!("openning {}", p.display());
        let mut m = MetaData::from_file(f).unwrap();
        Ok(m)
    }

    pub fn from_file(f: File) -> Result<MetaData, std::io::Error> {
        let mut m = MetaData::empty();
        //let f = File::open(path).unwrap();
        let mut fb = BufReader::new(f);
        loop {
            let mut s = String::new();
            match fb.read_line(&mut s) {
                Ok(v) => {
                    if v == 0 {
                        break;
                    }
                    m.process_line(s.as_str());
                },
                Err(e) => {
                    return Err(e);
                },
            }
        }
        Ok(m)
    }
}

impl fmt::Debug for MetaData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format_args!("title \"{}\" author \"{}\" digest {}", self.title(), self.author(), self.fingerprint()))
    }
}

#[cfg(test)]
mod tests {
    use super::MetaData;
    use std::path;
    use tempfile::NamedTempFile;
    use biblatex::EntryType;
    use std::fs::{
        File,
        write
    };
    use env_logger;

    #[test]
    fn test_metadata_create() {
        let s = path::Path::new("testdata/bitcoin.pdf");
        let meta = MetaData::from_xattr(s);
        assert_eq!(meta.dc.title, "Bitcoin: A Peer-to-Peer Electronic Cash System");
        assert_eq!(meta.dc.author, "Satoshi Nakamoto");
        assert_eq!(meta.fingerprint(), String::from("2ac531ee521cf93f8419c2018f770fbb42c65396178e079a416e7038d3f9ab9fc2c35c4d838bc8b5dd68f4c13759fe9cdf90a46528412fefe1294cb26beabf4e"));
    }

    #[test]
    fn test_metadata_set() {
        let digest_hex = "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e";
        let digest = hex::decode(&digest_hex).unwrap();

        let f = NamedTempFile::new_in(".").unwrap();
        let fp = f.path();
        let fps = String::from(fp.to_str().unwrap());

        let mut m = MetaData::new("foo", "bar", EntryType::Article, digest, Some(fps));
        m.set_subject("baz");
        m.set_mime_str("foo/bar");
        m.set_language("nb-NO");
        m.to_xattr(fp);
        
        let m_check = MetaData::from_xattr(fp);
        assert_eq!(m_check.title(), "foo");
        assert_eq!(m_check.author(), "bar");
        assert_eq!(m_check.fingerprint(), digest_hex);
        assert_eq!(m_check.typ(), EntryType::Article);
        assert_eq!(m_check.subject().unwrap(), "baz");
        assert_eq!(m_check.mime().unwrap(), "foo/bar");
        assert_eq!(m_check.language().unwrap(), "nb-NO");
    }

    #[test]
    fn test_metadata_file() {
        let f = File::open("testdata/meta.txt").unwrap();
        let m_check = MetaData::from_file(f).unwrap();
        assert_eq!(m_check.title(), "foo");
        assert_eq!(m_check.author(), "bar");
        assert_eq!(m_check.typ(), EntryType::Report);
        assert_eq!(m_check.subject().unwrap(), "baz");
        assert_eq!(m_check.mime().unwrap(), "text/plain");
        assert_eq!(m_check.language().unwrap(), "nb-NO");
    }

    #[test]
    fn test_metadata_xattr_magic() {
        let s = path::Path::new("testdata/bitcoin.pdf");
        let meta = MetaData::from_xattr(s);

        #[cfg(feature = "magic")]
        {
            assert_eq!(meta.mime().unwrap(), "application/pdf");
            let f = NamedTempFile::new_in(".").unwrap();
            let fp = f.path();
            write(&f, &[0, 1, 2, 3]);
            let meta_empty = MetaData::from_xattr(fp);
            assert_eq!(meta_empty.mime().unwrap(), "application/octet-stream"); 
        }
    }
}
