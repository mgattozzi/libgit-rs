use crate::{Blob, Mode, OID};
use bitvec::prelude::*;
use is_executable::IsExecutable;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
use std::{
  collections::BTreeMap,
  io,
  path::{Component, Path, PathBuf},
};
use walkdir::WalkDir;

pub struct Tree(BTreeMap<PathBuf, TreeItem>);

impl Tree {
  pub fn id(&self) -> OID {
    self.into()
  }

  pub fn from_dir(path: impl AsRef<Path>) -> Result<Self, io::Error> {
    let path = path.as_ref();
    if !path.is_dir() {
      todo!();
    }

    let mut tree = Tree(BTreeMap::new());
    for entry in WalkDir::new(path) {
      let entry = entry?;
      let diff = pathdiff::diff_paths(entry.path(), path).unwrap();
      let iter = path.components();

      // // A file with 1 component at least should exist
      // let mut item = tree.0.entry(iter.next().unwrap());
      // for component in path.components() {
      //   match component {
      //     Component::Normal(comp) => {
      //       item = item.entry(comp.into());
      //     }
      //     _ => unreachable!(),
      //   }
      // }

      println!("{}", diff.display());
    }

    Ok(tree)
  }

  pub fn as_bytes(&self) -> Vec<u8> {
    let content = self
      .0
      .iter()
      .map(|(path, item)| {
        // TODO: Make sure insertions have a non empty pathbuf and that they are
        // utf-8 compliant and that the path exists orrrrrrrrrrr just handle the
        // error.
        let file = path.file_name().unwrap().to_str().unwrap().as_bytes();
        let meta = path.metadata().unwrap();

        let mode = if meta.is_dir() {
          Mode::Directory
        } else if meta.file_type().is_symlink() {
          Mode::SymbolicLink
        } else if path.is_executable() {
          Mode::ExecutableFile
        } else {
          #[cfg(unix)]
          // Is the file group writeable bit set
          if meta.mode().view_bits::<Lsb0>()[5] {
            Mode::NonExecutableGroupWriteableFile
          } else {
            Mode::NonExecutableFile
          }

          #[cfg(windows)]
          Mode::NonExecutableFile
        };

        [
          mode.octal_string().as_bytes(),
          file,
          b"\0",
          &item.id().as_bytes(),
        ]
        .concat()
      })
      .flatten()
      .collect::<Vec<u8>>();
    [
      b"tree ",
      content.len().to_string().as_bytes(),
      b"\0",
      &content,
    ]
    .concat()
  }
}

pub enum TreeItem {
  Tree(Tree),
  Blob(Blob),
}

impl TreeItem {
  pub fn id(&self) -> OID {
    match self {
      Self::Blob(b) => b.id(),
      Self::Tree(t) => t.id(),
    }
  }

  pub fn is_tree(&self) -> bool {
    match self {
      Self::Blob(_) => false,
      Self::Tree(_) => true,
    }
  }

  pub fn is_blob(&self) -> bool {
    match self {
      Self::Blob(_) => true,
      Self::Tree(_) => false,
    }
  }
}

#[test]
fn from_dir() {
  use std::fs::{create_dir_all, write};
  let tmp_dir = tempdir::TempDir::new("tree_test").unwrap();
  let path = tmp_dir.path();
  let level1 = path.join("level1");
  let level2 = level1.join("level2");
  let level3 = level2.join("level3");
  create_dir_all(&level3).unwrap();
  write(path.join("a"), "testing 1 2 3").unwrap();
  write(path.join("b"), "testing 1 2 3").unwrap();
  write(path.join("c"), "testing 1 2 3").unwrap();
  write(path.join("d"), "testing 1 2 3").unwrap();
  write(level1.join("a"), "testing 1 2 3").unwrap();
  write(level1.join("b"), "testing 1 2 3").unwrap();
  write(level1.join("c"), "testing 1 2 3").unwrap();
  write(level1.join("d"), "testing 1 2 3").unwrap();
  write(level2.join("a"), "testing 1 2 3").unwrap();
  write(level2.join("b"), "testing 1 2 3").unwrap();
  write(level2.join("c"), "testing 1 2 3").unwrap();
  write(level2.join("d"), "testing 1 2 3").unwrap();
  write(level3.join("a"), "testing 1 2 3").unwrap();
  write(level3.join("b"), "testing 1 2 3").unwrap();
  write(level3.join("c"), "testing 1 2 3").unwrap();
  write(level3.join("d"), "testing 1 2 3").unwrap();

  let tree = Tree::from_dir(path).unwrap();
  panic!();
}
