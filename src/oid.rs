use crate::Blob;
use sha1::{Digest, Sha1};
use std::convert::TryInto;
use thiserror::Error;

/// An [`OID`] is the Object Identifier for a given git object which can be a
/// [`Blob`][crate::Blob], a Tree, or a Commit. This is a Sha1 sum of the object
/// that can be used to refer to the item in the Object Database.
#[derive(Debug, PartialEq, Eq)]
pub struct OID([u8; 20]);

impl OID {
  /// Get the Sha1 sum in a human readable hex format.
  pub fn as_hex(&self) -> String {
    hex::encode(self.0)
  }

  /// Make an OID from a human readable hex format. This function will fail if
  /// the length of the `&str` is not 40 characters long and that it's 40
  /// valid hex characters (as in 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, a, b, c, d, e, or f)
  pub fn from_hex(hex: &str) -> Result<Self, OIDError> {
    if hex.len() != 40 {
      return Err(OIDError::InvalidHex(HexErrorKind::TooShort(hex.len())));
    }
    let bytes =
      hex::decode(hex).map_err(|e| OIDError::InvalidHex(HexErrorKind::FromHexError(e)))?;
    // We know that this should always be 20 bytes long because we checked
    // above for the length of 40
    Ok(Self(bytes.try_into().unwrap()))
  }
}

impl From<Blob> for OID {
  fn from(blob: Blob) -> Self {
    let bytes = blob.as_bytes();
    let mut hasher = Sha1::new();
    hasher.update(bytes);
    Self(hasher.finalize().into())
  }
}

impl From<&Blob> for OID {
  fn from(blob: &Blob) -> Self {
    let bytes = blob.as_bytes();
    let mut hasher = Sha1::new();
    hasher.update(bytes);
    Self(hasher.finalize().into())
  }
}

impl From<&mut Blob> for OID {
  fn from(blob: &mut Blob) -> Self {
    let bytes = blob.as_bytes();
    let mut hasher = Sha1::new();
    hasher.update(bytes);
    Self(hasher.finalize().into())
  }
}

#[derive(Error, Debug)]
/// Errors related to operations done with the [`OID`] type
pub enum OIDError {
  #[error("invalid hex string used as input for OID. Reason was: {0}")]
  InvalidHex(HexErrorKind),
}

#[derive(Error, Debug)]
/// Errors related to hex based operations done with the [`OID`] type
pub enum HexErrorKind {
  #[error("{0}")]
  FromHexError(#[from] hex::FromHexError),
  #[error("hex string len was {0} instead of 40")]
  TooShort(usize),
}

#[test]
fn as_hex() {
  let oid = OID::from(&Blob::new("this is a test".as_bytes()));
  assert_eq!(&oid.as_hex(), "a8a940627d132695a9769df883f85992f0ff4a43");
}

#[test]
fn from_hex_too_short() {
  match OID::from_hex("aaa") {
    Ok(_) => unreachable!(),
    Err(OIDError::InvalidHex(HexErrorKind::TooShort(3))) => {}
    Err(_) => unreachable!(),
  }
}

#[test]
fn from_hex_invalid() {
  match OID::from_hex("012g34567891abcdefabcdef1234567890123456") {
    Ok(_) => panic!("OID was somehow valid when it should not be"),
    Err(OIDError::InvalidHex(HexErrorKind::FromHexError(
      hex::FromHexError::InvalidHexCharacter { c: 'g', index: 3 },
    ))) => {}
    Err(e) => panic!("OID failed with a different error: {}", e),
  }
}
