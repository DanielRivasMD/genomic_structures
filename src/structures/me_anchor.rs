////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  functions::flag_interpretor::SAMFlag,
  structures::{
    break_point::BreakPoint,
    cigar::CIGAR,
    orientation_enum::OrientationEnum,
  },
};
use crate::{
  ANCHOR_LIMIT,
  ME_LIMIT,
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

// load & update
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
  /// let mobel = "mobel77".to_string();
  /// let orientation = OrientationEnum::None;
  /// let position = 2099;
  /// let size = 11000.;
  ///
  /// let produced = MEAnchor::load(
  ///   CIGAR::load(cigar, position).expect("CIGAR loading failed!"),
  ///   flag,
  ///   mobel.clone(),
  ///   orientation,
  ///   position,
  ///   size,
  /// );
  ///
  /// let manual = MEAnchor {
  ///   breakpoint:  BreakPoint::new(),
  ///   cigar:       CIGAR::load(cigar, position).expect("CIGAR loading failed!"),
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
  /// let mobel = "mobel77".to_string();
  /// let orientation = OrientationEnum::None;
  /// let position = 2099;
  /// let size = 11000.;
  ///
  /// let mut produced = MEAnchor::new();
  /// produced.update(
  ///   CIGAR::load(cigar, position).expect("CIGAR loading failed!"),
  ///   flag,
  ///   mobel.clone(),
  ///   orientation,
  ///   position,
  ///   size,
  /// );
  ///
  /// let manual = MEAnchor {
  ///   breakpoint:  BreakPoint::new(),
  ///   cigar:       CIGAR::load(cigar, position).expect("CIGAR loading failed!"),
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

// calculate break point
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
  /// let mobel = "mobel77".to_string();
  /// let orientation = OrientationEnum::None;
  /// let position = 1;
  /// let size = 11000.;
  ///
  /// let mut loaded = MEAnchor::load(
  ///   CIGAR::load(cigar, position).expect("CIGAR loading failed!"),
  ///   flag,
  ///   mobel.clone(),
  ///   orientation,
  ///   position,
  ///   size,
  /// );
  ///
  /// loaded.calculate_break_point("MMMM0987654321B1234567890OOOOO");
  ///
  /// let mut manual = MEAnchor::new();
  /// manual.breakpoint = BreakPoint {
  ///   sequence:   "MMMM0987654321B1234567890".to_string(),
  ///   coordinate: 15.,
  /// };
  ///
  /// assert_eq!(loaded.breakpoint, manual.breakpoint);
  /// ```
  pub fn calculate_break_point(
    &mut self,
    sequence: &str,
  ) {
    // tag
    self.tag();
    if self.cigar.left_boundry <= 0 &&
      self.orientation == OrientationEnum::Upstream
    {
      self
        .breakpoint
        .update(sequence, self.cigar.left_boundry as f64);
    } else if self.cigar.right_boundry > self.size as i32 &&
      self.orientation == OrientationEnum::Downstream
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

// implement tag mobile element
impl TagME for MEAnchor {
  // read orientation
  fn read_orientation(&self) -> bool {
    self.interpret(5)
  }

  // upstream
  fn upstream(&mut self) {
    self.orientation = OrientationEnum::Upstream;
  }

  // downstream
  fn downstream(&mut self) {
    self.orientation = OrientationEnum::Downstream;
  }

  // reset
  fn reset_orientation(&mut self) {
    self.orientation = OrientationEnum::None;
  }

  // get size
  fn get_size(&self) -> f64 {
    self.size
  }

  // cigar left boundry
  fn get_cigar_left_boundry(&self) -> i32 {
    self.cigar.left_boundry
  }

  // cigar right boundry
  fn get_cigar_rigth_boundry(&self) -> i32 {
    self.cigar.right_boundry
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// tag
/// Tag mobile element.
pub trait TagME {
  ///
  /// Tag mobile element.
  ///
  /// # Examples
  ///
  /// ```
  /// use genomic_structures::{
  ///   BreakPoint,
  ///   MEAnchor,
  ///   OrientationEnum,
  ///   TagME,
  ///   CIGAR,
  /// };
  ///
  /// let cigar = "15S15M";
  /// let flag = 83;
  /// let mobel = "mobel77".to_string();
  /// let orientation = OrientationEnum::None;
  /// let position = 1;
  /// let size = 11000.;
  ///
  /// let mut loaded = MEAnchor::load(
  ///   CIGAR::load(cigar, position).expect("CIGAR loading failed!"),
  ///   flag,
  ///   mobel.clone(),
  ///   orientation,
  ///   position,
  ///   size,
  /// );
  ///
  /// loaded.tag();
  ///
  /// let mut manual = MEAnchor::new();
  /// manual.orientation = OrientationEnum::Upstream;
  ///
  /// assert_eq!(loaded.orientation, manual.orientation);
  /// ```
  fn tag(&mut self) {
    // upstream: read anchor reverse & mate unmapped
    if self.get_cigar_left_boundry() <= ME_LIMIT && self.read_orientation() {
      self.upstream();
    // upstream: read anchor & mate mapped reverse
    } else if self.get_cigar_rigth_boundry() != 0 &&
      self.get_cigar_rigth_boundry() <= ANCHOR_LIMIT &&
      !self.read_orientation()
    {
      self.upstream();
    // downstream: read anchor & mate unmapped
    } else if self.get_size() - self.get_cigar_rigth_boundry() as f64 <=
      ME_LIMIT.into() &&
      self.get_size() != 0. &&
      !self.read_orientation()
    {
      self.downstream();
    // downstream: read anchor reverse & mate mapped
    } else if self.get_size() - self.get_cigar_left_boundry() as f64 <=
      ANCHOR_LIMIT.into() &&
      self.read_orientation()
    {
      self.downstream();
    } else {
      self.reset_orientation();
    }
  }

  // read orientation
  fn read_orientation(&self) -> bool;

  // upstream
  fn upstream(&mut self);

  // downstream
  fn downstream(&mut self);

  // reset
  fn reset_orientation(&mut self);

  // get size
  fn get_size(&self) -> f64;

  // cigar left boundry
  fn get_cigar_left_boundry(&self) -> i32;

  // cigar right boundry
  fn get_cigar_rigth_boundry(&self) -> i32;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
