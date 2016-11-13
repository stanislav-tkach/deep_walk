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
    #[test]
    fn it_works() {}
}
