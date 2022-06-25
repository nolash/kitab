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
