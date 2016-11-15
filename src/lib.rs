use std::path::{Path, PathBuf};

pub struct DeepWalk {
    root: PathBuf,
}

impl DeepWalk {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        DeepWalk { root: root.as_ref().to_path_buf() }
    }
}

impl IntoIterator for DeepWalk {
    type Item = Result<Entry, Error>;
    type IntoIter = Iter;

    fn into_iter(self) -> Iter {
        Iter { root: self.root }
    }
}

pub struct Iter {
    root: PathBuf,
}

// TODO: Remove and implement Iterator for DeepWalk.
impl Iterator for Iter {
    type Item = Result<Entry, Error>;

    fn next(&mut self) -> Option<Result<Entry, Error>> {
        None
    }
}

pub struct Entry {
    //
}

pub struct Error {
    //
}

#[cfg(test)]
mod tests {
    use super::DeepWalk;
    use std::path::Path;

    fn get_test_roots() -> &'static[&'static str] {
        const DATA: &'static[&'static str] = &["", "a", "test", "eee/aaa", "some/long/path"];
        DATA
    }

    #[test]
    fn deep_walk_new() {
        for val in get_test_roots() {
            assert_eq!(DeepWalk::new(val).root, Path::new(val));
        }
    }

    #[test]
    fn deep_walk_into_iterator() {
        for val in get_test_roots() {
            assert_eq!(DeepWalk::new(val).into_iter().root, Path::new(val));
        }
    }
}
