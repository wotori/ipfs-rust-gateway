use std::borrow::Cow;
use std::path::{Path, PathBuf};

use rand::{self, Rng};

use rocket::request::FromParam;

/// Returns an instance of `PasteId` if the path segment is a valid ID.
/// Otherwise returns the invalid ID as the `Err` value.
impl<'a> FromParam<'a> for PasteId<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        param.chars().all(|c| c.is_ascii_alphanumeric())
            .then(|| PasteId(param.into()))
            .ok_or(param)
    }
}

/// A _probably_ unique paste ID.
pub struct PasteId<'a>(Cow<'a, str>);

impl PasteId<'_> {
    /// Generate a _probably_ unique ID with `size` characters. For readability,
    /// the characters used are from the sets [0-9], [A-Z], [a-z]. The
    /// probability of a collision depends on the value of `size` and the number
    /// of IDs generated thus far.
    pub fn new(size: usize) -> PasteId<'static> {
        const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }
        PasteId(Cow::Owned(id))
    }

    /// Returns the path to the paste in `upload/` corresponding to this ID.
    pub fn file_path(&self) -> PathBuf {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "files");
        Path::new(root).join(self.0.as_ref())
    }

    // As string
    /// Returns the path to the paste in `upload/` corresponding to this ID.
    pub fn file_name(&self) -> String {
        let a = "/files/".to_string();
        let name = self.0.as_ref();
        let name = format!("{}{}{}", env!("CARGO_MANIFEST_DIR"), a, name);
        println!("finally got this {}!", name);
        String::from(name)
    }
}