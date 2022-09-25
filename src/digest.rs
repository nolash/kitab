use std::marker::Copy;

use crate::error::ParseError;

use sha2::{
    Sha512,
    Sha256,
    Digest,
};

use log::error;

/// Encapsulations of supported digests for digest data.
pub enum RecordDigest {
    Sha512(Vec<u8>),
    Sha256(Vec<u8>),
    MD5(Vec<u8>),
    SwarmHash(Vec<u8>),
    Empty,
}

impl Clone for RecordDigest {
    fn clone(&self) -> RecordDigest {
        match self {
            RecordDigest::Empty => {
                RecordDigest::Empty
            },
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
        }
    }
}

impl RecordDigest {
    pub fn fingerprint(&self) -> Vec<u8> {
        match self {
            RecordDigest::Empty => {
                return vec!();
            },
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
        }
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
            let digest_hex = v.next().unwrap();
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
            let digest_hex = v.next().unwrap();
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
