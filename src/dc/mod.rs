use mime::Mime;
use unic_langid_impl::LanguageIdentifier;
use biblatex::EntryType;
use std::str::FromStr;


/// Represents the parts of the metadata schema covered by the Dublin Core vocabulary.
pub struct DCMetaData {
    /// Title of work represented by media.
    pub title: String,
    /// Author(s) of work represented by media. Multiple authors may be specified by separating
    /// them with comma.
    pub author: String,
    /// Type of work represented by media. Maps to bibtex entry types.
    pub typ: EntryType,
    /// Comma-separated keyword list describing the content.
    pub subject: Option<String>,
    /// MIME type of the media.
    pub mime: Option<Mime>,
    /// What language the work represented by this media file is in.
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
    /// Creates a new Dublin Core metadata part with minimal data.
    ///
    /// `title`, `author` and `entry_type` map to corresponding [DCMetaData](DCMetaData)
    /// properties.
    pub fn new(title: &str, author: &str, entry_type: EntryType) -> DCMetaData {
        DCMetaData{
            title: String::from(title),
            author: String::from(author),
            typ: entry_type,
            subject: None,
            mime: None,
            language: None,
        }
    }
}
