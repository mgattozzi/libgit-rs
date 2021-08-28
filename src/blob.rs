use crate::OID;
use bstr::{BStr, BString, ByteSlice};
use std::{fs, io, path::Path};

/// A [`Blob`] is a git object that represents a file in a git directory. For
/// instance all of the bytes that makes up the file that these docs for this
/// struct reside in, would be stored as a [`Blob`] on disk.
#[derive(Debug, PartialEq, Eq)]
pub struct Blob(BString);

impl Blob {
  /// Create a [`Blob`] given a set of bytes
  pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
    let bytes: Vec<u8> = bytes.into();
    Self(bytes.into())
  }

  /// Turn the [`Blob`] into the on disk representation stored in Object
  /// Database, which is in the form below where:
  /// - {content_len} is the length of the file contents that the [`Blob`]
  ///   holds as a string. For example `blob 10\0` is a valid header. Using
  ///   the byte representation of 10 would not be
  /// - {content} is the actual content of the file as bytes that this
  ///   [`Blob`] represents
  ///
  /// ```text
  /// blob {content_len}\0{content}
  /// ```
  ///
  /// Note that a [`Blob`] is stored on disk with zlib for compression and
  /// that this only represents the uncompressed form
  pub fn as_bytes(&self) -> Vec<u8> {
    [
      b"blob ",
      self.0.len().to_string().as_bytes(),
      b"\0",
      &self.0.as_bytes(),
    ]
    .concat()
  }

  /// Get the [`OID`] for the [`Blob`]
  pub fn id(&self) -> OID {
    self.into()
  }

  /// Get the size of the contents of the [`Blob`].
  pub fn size(&self) -> usize {
    self.0.len()
  }

  /// Access the contents of the [`Blob`].
  pub fn contents(&self) -> &BStr {
    &self.0.as_bstr()
  }

  /// Turn a file into a [`Blob`]. This is a convenience function to handle
  /// turning files in a git directory into a [`Blob`] for cases like creating
  /// a commit for the current working tree.
  pub fn from_file(path: impl AsRef<Path>) -> Result<Self, io::Error> {
    Ok(Self::new(fs::read(path.as_ref())?))
  }
}

#[test]
fn as_bytes() {
  let blob = Blob::new("this is a test".as_bytes());
  let bytes = &blob.as_bytes();
  assert_eq!("blob 14\0this is a test".as_bytes(), bytes);
}
#[test]
fn id() {
  let blob = Blob::new("this is a test".as_bytes());
  let oid = blob.id();
  assert_eq!(
    OID::from_hex("a8a940627d132695a9769df883f85992f0ff4a43").unwrap(),
    oid
  );
}
#[test]
fn size() {
  let blob = Blob::new("this is a test".as_bytes());
  let size = blob.size();
  assert_eq!(14, size);
}
#[test]
fn contents() {
  let blob = Blob::new("this is a test".as_bytes());
  let contents = blob.contents();
  assert_eq!("this is a test", contents);
}
#[test]
fn from_file() {
  let tmp_dir = tempdir::TempDir::new("blob_test").unwrap();
  let file_path = tmp_dir.path().join("from_file_test.txt");
  fs::write(&file_path, "this is a test").unwrap();
  let blob = Blob::from_file(file_path).unwrap();
  assert_eq!("this is a test", blob.contents());
  assert_eq!(14, blob.size());
  assert_eq!(
    OID::from_hex("a8a940627d132695a9769df883f85992f0ff4a43").unwrap(),
    blob.id()
  );
  assert_eq!("blob 14\0this is a test".as_bytes(), &blob.as_bytes());
}
