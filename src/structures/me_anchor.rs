////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  functions::flag_interpretor::{
    interpretor,
    SamFlag,
  },
  structures::cigar::CIGAR,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Mobile element anchor structure.
#[derive(Debug, new)]
pub struct MEAnchor {
  /// CIGAR.
  #[new(value = "CIGAR::new()")]
  pub cigar: CIGAR,

  /// Flag.
  #[new(default)]
  pub flag: i32,

  /// Mobile element.
  #[new(default)]
  pub mobel: String,

  /// Orientation.
  #[new(default)]
  pub orientation: String,

  /// Position.
  #[new(default)]
  pub position: i32,

  /// Size.
  #[new(default)]
  pub size: f64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl MEAnchor {
  // TODO: update tests
  /// Load vector of strings (line from a file) onto MEAnchor struct.
  ///
  /// # Examples
  ///
  /// ```
  /// use genomic_structures::MEAnchor;
  ///
  /// let loaded = MEAnchor::loader(
  ///   &vec!["", "75", "me11", "650", "100", "100M", "", "", "100"],
  ///   1000.,
  ///   &"FS5",
  /// );
  /// let manual = MEAnchor {
  ///   mobel:       "me11".to_string(),
  ///   size:        1000.,
  ///   flag:        75,
  ///   pos:         650,
  ///   cigar:       "100M".to_string(),
  ///   orientation: "FS5".to_string(),
  /// };
  ///
  /// assert_eq!(loaded.mobel, manual.mobel);
  /// assert_eq!(loaded.size, manual.size);
  /// assert_eq!(loaded.flag, manual.flag);
  /// assert_eq!(loaded.pos, manual.pos);
  /// assert_eq!(loaded.cigar, manual.cigar);
  /// assert_eq!(loaded.orientation, manual.orientation);
  /// ```
  pub fn loader(
    cigar: CIGAR,
    flag: i32,
    mobel: String,
    orientation: String,
    position: i32,
    size: f64,
  ) -> Self {
    Self {
      cigar,
      flag,
      mobel,
      orientation,
      position,
      size,
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl SamFlag for MEAnchor {
  fn interpretor(
    &self,
    p: usize,
  ) -> bool {
    interpretor(self.flag, p)
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
