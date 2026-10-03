//! In-memory file system standing in for the framework's `PakInterface` (`p_fopen`, ...).
//!
//! The original reads game files synchronously. Bevy systems must not block on I/O, so the
//! host reads the whole install (images, data, properties, sounds) into this map on a task
//! pool thread before the game boots; afterwards every "file read" in game code is a map
//! lookup. Paths are matched case-insensitively with `\` and `/` treated alike, as on
//! Windows. Writes (saves) are recorded and flushed to disk by the host.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Default, Debug, Clone)]
pub struct Vfs {
    files: HashMap<String, Vec<u8>>,
    /// Files written by the game since the last flush (key -> bytes).
    pub dirty: HashMap<String, Vec<u8>>,
    /// Files deleted by the game since the last flush.
    pub deleted: Vec<String>,
    pub root: PathBuf,
}

/// Normalized lookup key: lowercase, `/` separators, no leading `./`.
pub fn key(path: &str) -> String {
    let mut k = path.replace('\\', "/").to_ascii_lowercase();
    while let Some(rest) = k.strip_prefix("./") {
        k = rest.to_string();
    }
    k
}

impl Vfs {
    /// Reads every file under `root` (recursively). Meant to run on a task-pool thread.
    pub fn load_dir(root: &Path) -> std::io::Result<Vfs> {
        fn walk(base: &Path, dir: &Path, out: &mut HashMap<String, Vec<u8>>) -> std::io::Result<()> {
            for e in std::fs::read_dir(dir)? {
                let p = e?.path();
                if p.is_dir() {
                    // HD art (port addition) is loaded per image by `crate::sexy::hd`.
                    if dir == base && p.file_name().is_some_and(|n| n.eq_ignore_ascii_case("hd")) {
                        continue;
                    }
                    walk(base, &p, out)?;
                } else {
                    let rel = p.strip_prefix(base).unwrap().to_string_lossy().to_string();
                    out.insert(key(&rel), std::fs::read(&p)?);
                }
            }
            Ok(())
        }
        let mut files = HashMap::new();
        walk(root, root, &mut files)?;
        Ok(Vfs { files, dirty: HashMap::new(), deleted: Vec::new(), root: root.to_path_buf() })
    }

    pub fn from_files(files: impl IntoIterator<Item = (String, Vec<u8>)>) -> Vfs {
        Vfs { files: files.into_iter().map(|(k, v)| (key(&k), v)).collect(), ..Default::default() }
    }

    pub fn read(&self, path: &str) -> Option<&[u8]> {
        self.files.get(&key(path)).map(|v| v.as_slice())
    }

    /// `FindFirstFile` / `FindNextFile` on `dir\*<ext>`: the file names (lowercase) directly
    /// in `dir` ending in `ext`, in name order (NTFS returns names sorted, ignoring case).
    pub fn find_files(&self, dir: &str, ext: &str) -> Vec<String> {
        let d = format!("{}/", key(dir).trim_end_matches('/'));
        let ext = ext.to_ascii_lowercase();
        let mut v: Vec<String> = self
            .files
            .keys()
            .filter_map(|k| k.strip_prefix(&d))
            .filter(|n| !n.contains('/') && n.ends_with(&ext))
            .map(|n| n.to_string())
            .collect();
        v.sort();
        v
    }

    pub fn exists(&self, path: &str) -> bool {
        self.files.contains_key(&key(path))
    }

    pub fn write(&mut self, path: &str, data: Vec<u8>) {
        let k = key(path);
        self.files.insert(k.clone(), data.clone());
        self.dirty.insert(k, data);
    }

    /// Deletes a file; false when there was none.
    pub fn remove(&mut self, path: &str) -> bool {
        let k = key(path);
        self.dirty.remove(&k);
        let had = self.files.remove(&k).is_some();
        if had {
            self.deleted.push(k);
        }
        had
    }

    pub fn len(&self) -> usize {
        self.files.len()
    }
}
