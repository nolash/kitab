use std::io::Write;
use std::path::{
    PathBuf,
    Path,
};
use std::fs::File;

use crate::meta::MetaData;

/// Represents the filesystem storage location for metadata.
pub struct FileStore{
    path: PathBuf,
}

impl FileStore {
    /// Create new store.
    pub fn new(p: &Path) -> Self {
        FileStore{
            path: p.to_path_buf(),
        }
    }

    /// Generate new writer for adding / modifying a metadata entry in the store.
    pub fn writer(&self, entry: &MetaData) -> impl Write {
        let p = self.path.join(entry.fingerprint());
        File::create(&p).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use biblatex::EntryType;
    use tempfile::tempdir;
    use super::{
        FileStore,
        MetaData,
    };
    use crate::digest;
    use std::io::Write;

    #[test]
    fn test_writer() {
        let mut digest = Vec::with_capacity(64);
        digest.resize(64, 0x2a);
        let digest_sha = digest::from_vec(Vec::from(digest)).unwrap();
        let m = MetaData::new("foo", "bar", EntryType::Article, digest_sha, None);
        let dir = tempdir().unwrap();
        let fp = dir.path();
        let fs = FileStore::new(&fp);
        let mut w = fs.writer(&m);
        w.write(m.title().as_bytes());
    }
}
