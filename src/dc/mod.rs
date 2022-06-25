use mime::Mime;
use unic_langid_impl::LanguageIdentifier;


pub struct DCMetaData {
    pub title: String,
    pub author: String,
    pub subject: Option<String>,
    pub mime: Option<Mime>,
    pub language: Option<LanguageIdentifier>,
}

impl DCMetaData {
    pub fn new(title: &str, author: &str) -> DCMetaData {
        DCMetaData{
            title: String::from(title),
            author: String::from(author),
            subject: None,
            mime: None,
            language: None,
        }
    }
}
