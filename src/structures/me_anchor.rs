////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  functions::sam_flag::SAMFlag,
  structures::{
    cigar::CIGAR,
    orientation_enum::OrientationEnum,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Mobile element anchor structure.
#[derive(Debug, new, Clone, Default, PartialEq)]
pub struct MEAnchor {
  /// CIGAR string.
  #[new(value = "CIGAR::new()")]
  pub cigar: CIGAR,

  /// Alignment flag.
  #[new(default)]
  pub flag: i32,

  /// Mobile element alignment allocation.
  #[new(default)]
  pub mobel: String,

  /// Mobile element alignment orientation.
  #[new(default)]
  pub orientation: OrientationEnum,

  /// Alignment position.
  #[new(default)]
  pub position: i32,

  /// Mobile element size.
  #[new(default)]
  pub size: f64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// create
impl MEAnchor {
  ///
  /// Load values onto `MEAnchor`.
  ///
  /// # Parameters
  ///
  /// * `cigar` - CIGAR string.
  ///
  /// * `flag` - Alignment flag.
  ///
  /// * `mobel` - Mobile element alignment allocation.
  ///
  /// * `orientation` - Mobile element alignment orientation.
  ///
  /// * `position` - Alignment position.
  ///
  /// * `size` - Mobile element size.
  ///
  /// # Returns
  ///
  /// Return an instance of `MEAnchor` with SAM parameters.
  ///
  /// # Examples
  ///
  /// ```
  /// use genomic_structures::{
  ///   MEAnchor,
  ///   OrientationEnum,
  ///   CIGAR,
  /// };
  ///
  /// let cigar = "100M";
  /// let flag = 75;
  /// let mobel = String::from("mobel77");
  /// let orientation = OrientationEnum::None;
  /// let position = 2099;
  /// let size = 11000.;
  ///
  /// let produced = MEAnchor::load(
  ///   CIGAR::load(cigar, position).unwrap(),
  ///   flag,
  ///   mobel.clone(),
  ///   orientation,
  ///   position,
  ///   size,
  /// );
  ///
  /// let manual = MEAnchor {
  ///   cigar:       CIGAR::load(cigar, position).unwrap(),
  ///   flag:        flag,
  ///   mobel:       mobel.clone(),
  ///   orientation: OrientationEnum::None,
  ///   position:    position,
  ///   size:        size,
  /// };
  ///
  /// assert_eq!(produced, manual);
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

  ///
  /// Update values of `MEAnchor`.
  ///
  /// # Parameters
  ///
  /// * `cigar` - CIGAR string.
  ///
  /// * `flag` - Alignment flag.
  ///
  /// * `mobel` - Mobile element alignment allocation.
  ///
  /// * `orientation` - Mobile element alignment orientation.
  ///
  /// * `position` - Alignment position.
  ///
  /// * `size` - Mobile element size.
  ///
  /// # Returns
  ///
  /// Return an updated instance of `MEAnchor` with SAM parameters.
  ///
  /// # Examples
  ///
  /// ```
  /// use genomic_structures::{
  ///   MEAnchor,
  ///   OrientationEnum,
  ///   CIGAR,
  /// };
  ///
  /// let cigar = "100M";
  /// let flag = 75;
  /// let mobel = String::from("mobel77");
  /// let orientation = OrientationEnum::None;
  /// let position = 2099;
  /// let size = 11000.;
  ///
  /// let mut produced = MEAnchor::new();
  /// produced.update(
  ///   CIGAR::load(cigar, position).unwrap(),
  ///   flag,
  ///   mobel.clone(),
  ///   orientation,
  ///   position,
  ///   size,
  /// );
  ///
  /// let manual = MEAnchor {
  ///   cigar:       CIGAR::load(cigar, position).unwrap(),
  ///   flag:        flag,
  ///   mobel:       mobel.clone(),
  ///   orientation: OrientationEnum::None,
  ///   position:    position,
  ///   size:        size,
  /// };
  ///
  /// assert_eq!(produced, manual);
  /// ```
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
impl SAMFlag for MEAnchor {
  fn get_flag(&self) -> i32 {
    self.flag
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
