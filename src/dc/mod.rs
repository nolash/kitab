use mime::Mime;
use unic_langid_impl::LanguageIdentifier;
use biblatex::EntryType;
use std::str::FromStr;


pub struct DCMetaData {
    pub title: String,
    pub author: String,
    pub typ: EntryType,
    pub subject: Option<String>,
    pub mime: Option<Mime>,
    pub language: Option<LanguageIdentifier>,
}

pub const DC_IRI_TITLE: &str = "https://purl.org/dc/terms/title";
pub const DC_IRI_CREATOR: &str = "https://purl.org/dc/terms/creator";
pub const DC_IRI_SUBJECT: &str = "https://purl.org/dc/terms/subject";
pub const DC_IRI_LANGUAGE: &str = "https://purl.org/dc/terms/language";
pub const DC_IRI_TYPE: &str = "https://purl.org/dc/terms/type";
pub const DC_IRI_MEDIATYPE: &str = "https://purl.org/dc/terms/MediaType";
pub const DC_XATTR_TITLE: &str = "user.dcterms:title";
pub const DC_XATTR_CREATOR: &str = "user.dcterms:creator";
pub const DC_XATTR_SUBJECT: &str = "user.dcterms:subject";
pub const DC_XATTR_LANGUAGE: &str = "user.dcterms:language";
pub const DC_XATTR_TYPE: &str = "user.dcterms:type";
pub const DC_XATTR_MEDIATYPE: &str = "user.dcterms:MediaType";

impl DCMetaData {
    pub fn new(title: &str, author: &str, typ: EntryType) -> DCMetaData {
        DCMetaData{
            title: String::from(title),
            author: String::from(author),
            typ: typ,
            subject: None,
            mime: None,
            language: None,
        }
    }
}
