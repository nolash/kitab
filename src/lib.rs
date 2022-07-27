//! kitab is a CLI tool to manage backup of media metadata information, primarily intended for
//! bibliographical sources.
//!
//! The tool can recursively apply metadata as extended attributes on all files in a filesystem
//! location whose digests match the respective keys of the metadata.
//!
//! Also, metadata can be imported from the same extended file attributes, as well as
//! files containing [bibtex](http://www.bibtex.org/Format/) entries and
//! entries in kitab's native store format.
//!
//! ## Usage examples
//!
//! ``` ignore;
//! ## import rdf-turtle entries from file to store.
//! $ kitab import source.ttl
//!
//! ## import bibtex entries from file to store
//! $ kitab import source.bib
//!  
//! ## import entries from any valid source under the given path
//! $ kitab import /path/to/metadata_and_or_media_files
//!
//! ## apply metadata on files matching digests in store
//! $ kitab apply /path/to/media_files
//! ```
//!
//! ## Native store format
//!
//! The native data format is [rdf-turtle](https://www.w3.org/TR/turtle/), currently limited to a
//! subset of the [DublinCore](https://www.dublincore.org/specifications/dublin-core/dcmi-terms/) vocabulary.
//!
//! The subject of all entries is a URN specifying the digest of the matching file, in the format
//! (digest hex for illustration purpose only):
//!
//! ``` ignore;
//! <URN:sha256:2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae> predicate object
//! [...]
//! ```
//! Please forgive the lack of a schema describing the data. It will follow.
//!
//! ## Store location
//!
//! Metadata files are stored under `~/.local/share/kitab/idx/<hex>` where `<hex>` is the
//! (lowercase) digest hex matching the URN in the record.
//!
//! ## Supported digests
//!
//! * `SHA512` (native)
//! * `SHA256`
//!
//! Metadata imported from extended attributes will use the `SHA512` digest of the file as the
//! storage key.
//!
//! ## Example
//!
//! The rust crate author's [PDF
//! copy](https://g33k.holbrook.no/b1674191a88ec5cdd733e4240a81803105dc412d6c6708d53ab94fc248f4f553) of the _Bitcoin whitepaper_ has `SHA256` hash
//! `b1674191a88ec5cdd733e4240a81803105dc412d6c6708d53ab94fc248f4f553`
//!
//! The rdf-turtle record for this document could be:
//!
//! ``` ignore;
//! @prefix dcterms: <https://purl.org/dc/terms/> .
//! @prefix dcmi: <https://purl.org/dc/dcmi/> .
//!
//! <URN:sha256:b1674191a88ec5cdd733e4240a81803105dc412d6c6708d53ab94fc248f4f553>
//!     dcterms:title "Bitcoin: A Peer-to-Peer Electronic Cash System" ;
//!     dcterms:subject "bitcoin,cryptocurrency,cryptography" ;
//!	    dcterms:creator "Satoshi Nakamoto" ;
//!	    dcterms:type "article" ;
//!	    dcterms:MediaType "application/pdf" ;
//!	    dcterms:language "en" .
//! ```
//!
//! After applying the metadata to the document itself, the extended attributes could look like
//! this:
//!
//! ``` ignore;
//! $ getfattr -d pub/papers/bitcoin.pdf 
//! user.dcterms:creator="Satoshi Nakamoto"
//! user.dcterms:language="en"
//! user.dcterms:subject="bitcoin,cryptocurrency"
//! user.dcterms:title="Bitcoin: A Peer-to-Peer Electronic Cash System"
//! user.dcterms:type="article"
//! ```
//!
//! ### Optional: File magic
//!
//! If built with the `magic` feature, an attempt will be made to determine the media type for each
//! file, and include the `dcterms:MediaType` predicate accordingly.
//!
//! Without the `magic` feature, the `dcterms.MediaType` will not be included in the metadata
//! record.
//!
//! ## Debugging
//!
//! `kitab` uses [env_logger](env_logger). Loglevel can be set using the
//! `RUST_LOG` environment variable to see what's going on when running the tool.
#![crate_name = "kitab"]

pub mod meta;

pub mod dc;

pub mod store;

pub mod rdf;

pub mod biblatex;

pub mod error;

pub mod digest;

#[cfg(test)]
mod tests {
    use env_logger;

    #[test]
    fn test_setup_env_logger() {
        env_logger::init();
    }
}
