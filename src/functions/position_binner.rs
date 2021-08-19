////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::BIN_SIZE;

////////////////////////////////////////////////////////////////////////////////////////////////////

///
/// Bin alignment position.
///
/// # Returns
/// Return binned position.
///
/// # Examples
///
/// ```
/// use genomic_structures::bin;
///
/// assert_eq!(bin(2099), 2000);
/// ```
pub fn bin(position: i32) -> i32 {
  let binned = position % BIN_SIZE;
  position - binned
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Anchor methods.
pub trait Anchor {
  ///
  /// Bin alignment position.
  ///
  /// # Returns
  /// Return binned position.
  ///
  /// # Examples
  ///
  /// ```
  /// use genomic_structures::bin;
  ///
  /// assert_eq!(bin(2099), 2000);
  /// ```
  fn bin(&self) -> i32 {
    let binned = self.get_position() % BIN_SIZE;
    self.get_position() - binned
  }

  /// Retrieve position from struct.
  fn get_position(&self) -> i32;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
