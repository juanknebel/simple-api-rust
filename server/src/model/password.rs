#[cfg(test)]
use mockall::automock;
use sha2::{Digest, Sha256};

#[cfg_attr(test, automock)]
pub trait PasswordHasher {
  /// Calculate hash for any given string using SHA256.
  ///
  /// # Arguments
  /// * `some_string` - The string to hashed.
  ///
  /// # Return
  /// * A string that represents the hash of the given one.
  fn hash(&self, some_string: &str) -> String;
}

#[derive(Default)]
pub struct SimpleHasher;

impl PasswordHasher for SimpleHasher {
  fn hash(&self, some_string: &str) -> String {
    let mut hasher = Sha256::new();
    // hasher.update(password.as_ref());
    hasher.update(<String as AsRef<[u8]>>::as_ref(&some_string.to_string()));
    format!("{:X}", hasher.finalize())
  }
}
