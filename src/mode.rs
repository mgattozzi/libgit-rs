use std::fmt;

#[repr(u16)]
#[derive(Clone, Copy, Debug)]
pub enum Mode {
  Directory = 0o040000,
  NonExecutableFile = 0o100644,
  NonExecutableGroupWriteableFile = 0o100664,
  ExecutableFile = 0o100755,
  SymbolicLink = 0o120000,
  GitLink = 0o160000,
}

impl Mode {
  pub fn octal_string(&self) -> String {
    format!("{:06o}", self)
  }
}

impl fmt::Octal for Mode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Octal::fmt(&(*self as u16), f)
  }
}

#[test]
fn mode_octal_strings() {
  assert_eq!(&Mode::Directory.octal_string(), "040000");
  assert_eq!(&Mode::NonExecutableFile.octal_string(), "100644");
  assert_eq!(
    &Mode::NonExecutableGroupWriteableFile.octal_string(),
    "100664"
  );
  assert_eq!(&Mode::ExecutableFile.octal_string(), "100755");
  assert_eq!(&Mode::SymbolicLink.octal_string(), "120000");
  assert_eq!(&Mode::GitLink.octal_string(), "160000");
}
