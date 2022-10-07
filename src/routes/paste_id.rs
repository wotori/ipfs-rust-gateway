extern crate dirs;

use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::string::ToString;
use rand::{self, Rng};

pub struct PasteId<'a>(Cow<'a, str>);

impl PasteId<'_> {
    pub fn new(size: usize) -> PasteId<'static> {
        const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }
        PasteId(Cow::Owned(id))
    }

    pub fn file_path(&self) -> PathBuf {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "files");
        Path::new(root).join(self.0.as_ref())
    }

    // As string
    pub fn file_name(&self) -> String {
        let path = "/files/".to_string();
        let generated_name = self.0.as_ref();
        let name = format!("{}{}{}", env!("CARGO_MANIFEST_DIR"), path, generated_name);
        println!("finally got this path: {}", name);
        String::from(name)
    }
}