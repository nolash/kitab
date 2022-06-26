use std::io::Write;
use std::path::{
    PathBuf,
    Path,
};
use std::fs::File;

use crate::meta::MetaData;

pub struct FileStore{
    path: PathBuf,
}

impl FileStore {
    pub fn new(p: &Path) -> Self {
        FileStore{
            path: p.to_path_buf(),
        }
    }

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
    use std::io::Write;

    #[test]
    fn test_writer() {
        let mut digest = Vec::with_capacity(64);
        digest.resize(64, 0x2a);
        let m = MetaData::new("foo", "bar", EntryType::Article, Vec::from(digest), None);
        let dir = tempdir().unwrap();
        let fp = dir.path();
        let fs = FileStore::new(&fp);
        let mut w = fs.writer(&m);
        w.write(m.title().as_bytes());
    }
}
