use crate::error::ParseError;

use sha2::{
    Sha512,
    Sha256,
    Digest,
};

use log::error;

pub enum RecordDigest {
    Sha512(Vec<u8>),
    Sha256(Vec<u8>),
    SwarmHash(Vec<u8>),
    Empty,
}



pub fn from_vec(v: Vec<u8>) -> Result<RecordDigest, ParseError> {
    let sz = Sha512::output_size();
    if v.len() != sz {
        return Err(ParseError);
    }
    Ok(RecordDigest::Sha512(v))
}

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
                    return Err(ParseError);
                },
            }
        },
        Some("sha256") => {
            let digest_hex = v.next().unwrap();
            let digest = hex::decode(digest_hex).unwrap();

            let sz = Sha256::output_size();
            if digest.len() != sz {
                return Err(ParseError);
            }

            RecordDigest::Sha256(digest)
        },
        Some("bzz") => {
            let digest_hex = v.next().unwrap();
            let digest = hex::decode(digest_hex).unwrap();

            if digest.len() != 32 {
                return Err(ParseError);
            }
            
            RecordDigest::SwarmHash(digest)
        },
        Some("") => {
            RecordDigest::Empty
        },
        Some(_) => {
            return Err(ParseError);
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
