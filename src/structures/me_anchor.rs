////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  functions::{
    flag_interpretor::interpret,
    sam_flag::SamFlag,
  },
  structures::{
    cigar::CIGAR,
    orientation_enum::OrientationEnum,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Mobile element anchor structure.
#[derive(Debug, new, Clone, Default, PartialEq)]
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
  pub orientation: OrientationEnum,

  /// Position.
  #[new(default)]
  pub position: i32,

  /// Size.
  #[new(default)]
  pub size: f64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// create
impl MEAnchor {
 // TODO: update tests
//  /// use genomic_structures::MEAnchor;
//  ///
//  /// let loaded = MEAnchor::loader(
//  ///   &vec!["", "75", "me11", "650", "100", "100M", "", "", "100"],
//  ///   1000.,
//  ///   &"FS5",
//  /// );
//  /// let manual = MEAnchor {
//  ///   mobel:       "me11".to_string(),
//  ///   size:        1000.,
//  ///   flag:        75,
//  ///   pos:         650,
//  ///   cigar:       "100M".to_string(),
//  ///   orientation: "FS5".to_string(),
//  /// };
//  ///
//  /// assert_eq!(loaded.mobel, manual.mobel);
//  /// assert_eq!(loaded.size, manual.size);
//  /// assert_eq!(loaded.flag, manual.flag);
//  /// assert_eq!(loaded.pos, manual.pos);
//  /// assert_eq!(loaded.cigar, manual.cigar);
//  /// assert_eq!(loaded.orientation, manual.orientation);
  /// Load vector of strings (line from a file) onto MEAnchor struct.
  ///
  /// # Examples
  ///
  /// ```
  /// ```
  pub fn load(
    cigar: CIGAR,
    flag: i32,
    mobel: String,
    orientation: OrientationEnum,
    position: i32,
    size: f64,
  ) -> Self {
    let mut me_anchor = Self::new();
    me_anchor.update(cigar, flag, mobel, orientation, position, size);
    me_anchor
  }

  pub fn update(
    &mut self,
    cigar: CIGAR,
    flag: i32,
    mobel: String,
    orientation: OrientationEnum,
    position: i32,
    size: f64,
  ) {
    self.cigar = cigar;
    self.flag = flag;
    self.mobel = mobel;
    self.orientation = orientation;
    self.position = position;
    self.size = size;
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// implement SAM flag
impl SamFlag for MEAnchor {
  fn interpret(
    &self,
    p: usize,
  ) -> bool {
    interpret(self.flag, p)
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
