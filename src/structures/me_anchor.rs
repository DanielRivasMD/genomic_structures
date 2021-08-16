////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  functions::sam_flag::SAMFlag,
  structures::{
    break_point::BreakPoint,
    cigar::CIGAR,
    orientation_enum::OrientationEnum,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Mobile element anchor structure.
#[derive(Debug, new, Default, PartialEq)]
pub struct MEAnchor {
  /// Breakpoint.
  #[new(default)]
  pub breakpoint: BreakPoint,

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
  ///   BreakPoint,
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
  ///   breakpoint:  BreakPoint::new(),
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
  ///   BreakPoint,
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
  ///   breakpoint:  BreakPoint::new(),
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

impl MEAnchor {
  ///
  /// Calculate break point.
  ///
  /// # Parameters
  ///
  /// * `sequence` - Original read sequence.
  ///
  /// # Examples
  ///
  /// ```
  /// use genomic_structures::{
  ///   BreakPoint,
  ///   MEAnchor,
  ///   OrientationEnum,
  ///   CIGAR,
  /// };
  ///
  /// let cigar = "15S15M";
  /// let flag = 83;
  /// let mobel = String::from("mobel77");
  /// let orientation = OrientationEnum::None;
  /// let position = 1;
  /// let size = 11000.;
  ///
  /// let mut me_anchor = MEAnchor::load(
  ///   CIGAR::load(cigar, position).unwrap(),
  ///   flag,
  ///   mobel.clone(),
  ///   orientation,
  ///   position,
  ///   size,
  /// );
  ///
  /// me_anchor.calculate_break_point("MMMM0987654321B1234567890OOOOO");
  ///
  /// let mut manual = MEAnchor::new();
  /// manual.breakpoint = BreakPoint {
  ///   sequence:   String::from("MMMM0987654321B1234567890"),
  ///   coordinate: 15.,
  /// };
  ///
  /// assert_eq!(me_anchor.breakpoint, manual.breakpoint);
  /// ```
  pub fn calculate_break_point(
    &mut self,
    sequence: &str,
  ) {
    // TODO: consider implementing tagging
    if self.cigar.left_boundry <= 0 && self.interpret(5) {
      self
        .breakpoint
        .update(sequence, self.cigar.left_boundry as f64);
    } else if self.cigar.right_boundry > self.size as i32 && !self.interpret(5)
    {
      self
        .breakpoint
        .update(sequence, self.cigar.right_boundry as f64 - self.size);
    }
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
