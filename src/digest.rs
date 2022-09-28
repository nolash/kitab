use std::marker::Copy;
use std::io::Read;
use std::fmt;
use std::str::FromStr;

use crate::error::ParseError;

use sha2::{
    Sha512,
    Sha256,
    Digest,
};

use log::error;

#[derive(Copy, Clone)]
pub enum DigestType {
    Sha512,
    #[cfg(feature="digest_md5")]
    MD5,
}

impl FromStr for DigestType {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<DigestType, Self::Err> {
        match s {
            "md5" => {
                return Ok(DigestType::MD5);
            },
            "sha512" => {
                return Ok(DigestType::Sha512);
            },
            _ => {
                return Err(ParseError::new("Unknown digest string"));
            },
        };
    }
}

impl DigestType {
    pub fn digest_for(&self, f: impl Read) -> RecordDigest {
        RecordDigest::Empty 
    }
}

/// Encapsulations of supported digests for digest data.
pub enum RecordDigest {
    Sha512(Vec<u8>),
    Sha256(Vec<u8>),
    MD5(Vec<u8>),
    SwarmHash(Vec<u8>),
    EmptyWithType(DigestType),
    Empty,
}

impl Clone for RecordDigest {
    fn clone(&self) -> RecordDigest {
        match self {
            RecordDigest::Sha512(v) => {
                RecordDigest::Sha512(v.to_vec())
            },
            RecordDigest::Sha256(v) => {
                RecordDigest::Sha256(v.to_vec())
            },
            RecordDigest::MD5(v) => {
                RecordDigest::MD5(v.to_vec())
            },
            RecordDigest::SwarmHash(v) => {
                RecordDigest::SwarmHash(v.to_vec())
            },
            _ => {
                RecordDigest::Empty
            },
        }
    }
}

impl RecordDigest {
    pub fn fingerprint(&self) -> Vec<u8> {
        match self {
            RecordDigest::Sha512(v) => {
                return v.to_vec();
            },
            RecordDigest::Sha256(v) => {
                return v.to_vec();
            },
            RecordDigest::MD5(v) => {
                return v.to_vec();
            },
            RecordDigest::SwarmHash(v) => {
                return v.to_vec();
            },
            _ => {
                return vec!()
            },
        }
    }

    /// Returns the digest value of the media as a hex-encoded string.
    ///
    /// TODO: implememt in fmt for digest instead
    pub fn urn(&self) -> String {
        match self {
            RecordDigest::Sha512(v) => {
                return String::from("sha512:") + hex::encode(&v).as_str();
            },
            RecordDigest::Sha256(v) => {
                return String::from("sha256:") + hex::encode(&v).as_str();
            },
            RecordDigest::MD5(v) => {
                return String::from("md5:") + hex::encode(&v).as_str();
            },
            RecordDigest::SwarmHash(v) => {
                return hex::encode(&v);
            },
            _ => {
                return String::new();
            },
        }
    }
}

impl fmt::Debug for RecordDigest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.urn())
    }
}

/// Create a [RecordDigest::Sha512](RecordDigest::Sha512) instance from the raw digest data.
///
/// Will fail if digest has incorrect length.
pub fn from_vec(v: Vec<u8>) -> Result<RecordDigest, ParseError> {
    let sz = Sha512::output_size();
    if v.len() != sz {
        return Err(ParseError::new("invalid digest size"));
    }
    Ok(RecordDigest::Sha512(v))
}

/// Create a [RecordDigest](RecordDigest) instance corresponding to the URN digest scheme.
///
/// Valid URN schemes and their corresponding enumerated values are:
/// 
/// * `sha512` -> [RecordDigest::Sha512](RecordDigest::Sha512])
/// * `sha256` -> [RecordDigest::Sha256](RecordDigest::Sha256])
/// * `bzz` -> [RecordDigest::SwarmHash](RecordDigest::SwarmHash])
pub fn from_urn(urn: &str) -> Result<RecordDigest, ParseError> {
    let mut v = urn.split(":");
    let r = match v.next() {
        Some("sha512") => {
            let digest_hex = match v.next() {
                Some(r) => {
                    r
                },
                None => {
                    return Err(ParseError::new("not a valid digest urn"));
                },
            };
            let digest = hex::decode(digest_hex).unwrap();
            match from_vec(digest) {
                Ok(vv) => {
                    vv
                },
                Err(e) => {
                    return Err(ParseError::new("invalid sha512 digest"));
                },
            }
        },
        Some("sha256") => {
            let digest_hex = v.next().unwrap();
            let digest = hex::decode(digest_hex).unwrap();

            let sz = Sha256::output_size();
            if digest.len() != sz {
                return Err(ParseError::new("invalid sha256 digest"));
            }

            RecordDigest::Sha256(digest)
        },
        Some("md5") => {
            let digest_hex = match v.next() {
                Some(r) => {
                    r
                },
                None => {
                    return Err(ParseError::new("not a valid digest urn"));
                },
            };
            let digest = hex::decode(digest_hex).unwrap();

            if digest.len() != 16 {
                return Err(ParseError::new("invalid md5 digest"));
            }

            RecordDigest::MD5(digest)
        },
        Some("bzz") => {
            let digest_hex = v.next().unwrap();
            let digest = hex::decode(digest_hex).unwrap();

            if digest.len() != 32 {
                return Err(ParseError::new("invalid bzz digest"));
            }
            
            RecordDigest::SwarmHash(digest)
        },
        Some("") => {
            RecordDigest::Empty
        },
        Some(_) => {
            return Err(ParseError::new("unknown digest type"));
        },
        None => {
            RecordDigest::Empty
        },
    };
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::from_urn;
    use super::ParseError;

    #[test]
    fn test_digest_urn_parse() {
        match from_urn("sha512:deadbeef") {
            Ok(v) => {
                panic!("expected fail");
            },
            _ => {},
        };
        match from_urn("sha512:deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef") {
            Ok(v) => {},
            _ => {
                panic!("expected pass");
            },
        };
        match from_urn("sha256:deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef") {
            Ok(v) => {},
            _ => {
                panic!("expected pass");
            },
        };
        match from_urn("bzz:deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef") {
            Ok(v) => {},
            _ => {
                panic!("expected pass");
            },
        };
        match from_urn("foo:deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef") {
            Ok(v) => {
                panic!("expected fail");
            },
            _ => {},
        };
        match from_urn("foo:deadbeef") {
            Ok(v) => {
                panic!("expected fail");
            },
            _ => {},
        };
        match from_urn("") {
            Ok(v) => {},
            _ => {
                panic!("expected pass");
            },
        };
    }
}
