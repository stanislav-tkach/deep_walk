use std::path::{Path, PathBuf};
use std::fs::{self, ReadDir, DirEntry};
use std::io::Error;

pub struct DeepWalk {
    root: PathBuf,
}

impl DeepWalk {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        DeepWalk { root: root.as_ref().to_path_buf() }
    }
}

impl IntoIterator for DeepWalk {
    type Item = Result<DirEntry, Error>;
    type IntoIter = Iter;

    fn into_iter(self) -> Iter {
        Iter { root: Some(self.root), dirs: Vec::new() }
    }
}

pub struct Iter {
    root: Option<PathBuf>,
    dirs: Vec<ReadDir>,
}

// TODO: Remove and implement Iterator for DeepWalk.
impl Iterator for Iter {
    type Item = Result<DirEntry, Error>;

    fn next(&mut self) -> Option<Result<DirEntry, Error>> {
        if let Some(path) = self.root.take() {
            match fs::read_dir(path) {
                Ok(dir) => self.dirs.push(dir),
                Err(err) => return Some(Err(err)),
            }
        }

        while !self.dirs.is_empty() {
            // TODO: FIXME.
            break;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::DeepWalk;
    use std::path::{Path, PathBuf};

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
            assert_eq!(DeepWalk::new(val).into_iter().root, Some(PathBuf::from(val)));
        }
    }
}
