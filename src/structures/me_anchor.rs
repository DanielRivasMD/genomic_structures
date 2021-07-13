////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::functions::flag_interpretor::{
  interpretor,
  SamFlag,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Mobile element anchor structure.
#[derive(Debug, new, Default)]
pub struct MEAnchor {
  /// Mobile element.
  #[new(default)]
  pub mobel: String,

  /// Size.
  #[new(default)]
  pub size: f64,

  /// Flag.
  #[new(default)]
  pub flag: i32,

  /// Position.
  #[new(default)]
  pub pos: i32,

  /// CIGAR.
  #[new(default)]
  pub cigar: String,

  /// Orientation.
  #[new(default)]
  pub orientation: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl MEAnchor {
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
    file_line: &[&str],
    mobile_size: f64,
    mobile_orientation: &str,
  ) -> Self {
    Self {
      mobel:       file_line[2].to_string(),
      size:        mobile_size,
      flag:        file_line[1].parse::<i32>().unwrap(),
      pos:         file_line[3].parse::<i32>().unwrap(),
      cigar:       file_line[5].to_string(),
      orientation: (&mobile_orientation).to_string(),
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
