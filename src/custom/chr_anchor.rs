////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::fmt;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  function::{
    flag_interpretor::SAMFlag,
    position_binner::Anchor,
  },
  custom::{
    anchor_enum::AnchorEnum,
    cigar::CIGAR,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Chromosomal anchor structure.
#[derive(Debug, new, Default, PartialEq)]
pub struct ChrAnchor {
  /// Anchoring orientation.
  #[new(default)]
  pub anchor: AnchorEnum,

  /// CIGAR string.
  #[new(value = "CIGAR::new()")]
  pub cigar: CIGAR,

  /// Chromosomal alignment allocation.
  #[new(default)]
  pub chr: String,

  /// Alignment flag.
  #[new(default)]
  pub flag: i32,

  /// Mapping quality (MAPQ).
  #[new(default)]
  pub mapq: i32,

  /// Alignment position.
  #[new(default)]
  pub position: i32,

  /// Template length (TLEN).
  #[new(default)]
  pub tlen: i32,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// load & update
impl ChrAnchor {
  ///
  /// Load values onto `ChrAnchor`.
  ///
  /// # Parameters
  ///
  /// * `cigar` - CIGAR string.
  ///
  /// * `chr` - Chromosomal alignment allocation.
  ///
  /// * `flag` - Alignment flag.
  ///
  /// * `mapq` - Mapping quality (MAPQ).
  ///
  /// * `position` - Alignment position.
  ///
  /// * `tlen` - Template length (TLEN).
  ///
  /// # Returns
  ///
  /// Return an instance of `ChrAnchor` with SAM parameters.
  ///
  /// # Examples
  ///
  /// ```
  /// use genomic_structures::{
  ///   AnchorEnum,
  ///   ChrAnchor,
  ///   CIGAR,
  /// };
  ///
  /// let cigar = "100M";
  /// let chr = "chr7".to_string();
  /// let flag = 56;
  /// let position = 2099;
  /// let mapq = 60;
  /// let tlen = 100;
  ///
  /// let produced = ChrAnchor::load(
  ///   CIGAR::load(cigar, position).expect("CIGAR loading failed!"),
  ///   chr.clone(),
  ///   flag,
  ///   mapq,
  ///   position,
  ///   tlen,
  /// );
  ///
  /// let manual = ChrAnchor {
  ///   anchor:   AnchorEnum::None,
  ///   cigar:    CIGAR::load(cigar, position).expect("CIGAR loading failed!"),
  ///   chr:      chr.clone(),
  ///   flag:     flag,
  ///   position: position,
  ///   mapq:     mapq,
  ///   tlen:     tlen,
  /// };
  ///
  /// assert_eq!(produced, manual);
  /// ```
  pub fn load(
    cigar: CIGAR,
    chr: String,
    flag: i32,
    mapq: i32,
    position: i32,
    tlen: i32,
  ) -> Self {
    let mut chr_anchor = Self::new();
    chr_anchor.update(cigar, chr, flag, mapq, position, tlen);
    // TODO: ignore anchor for now anchor: AnchorEnum::None,
    chr_anchor
  }

  ///
  /// Update values of `ChrAnchor`.
  ///
  /// # Parameters
  ///
  /// * `cigar` - CIGAR string.
  ///
  /// * `chr` - Chromosomal alignment allocation.
  ///
  /// * `flag` - Alignment flag.
  ///
  /// * `mapq` - Mapping quality (MAPQ).
  ///
  /// * `position` - Alignment position.
  ///
  /// * `tlen` - Template length (TLEN).
  ///
  /// # Returns
  ///
  /// Return an updated instance of `ChrAnchor` with SAM parameters.
  ///
  /// # Examples
  ///
  /// ```
  /// use genomic_structures::{
  ///   AnchorEnum,
  ///   ChrAnchor,
  ///   CIGAR,
  /// };
  ///
  /// let cigar = "100M";
  /// let chr = "chr7".to_string();
  /// let flag = 56;
  /// let position = 2099;
  /// let mapq = 60;
  /// let tlen = 100;
  ///
  /// let mut produced = ChrAnchor::new();
  /// produced.update(
  ///   CIGAR::load(cigar, position).expect("CIGAR loading failed!"),
  ///   chr.clone(),
  ///   flag,
  ///   mapq,
  ///   position,
  ///   tlen,
  /// );
  ///
  /// let manual = ChrAnchor {
  ///   anchor:   AnchorEnum::None,
  ///   cigar:    CIGAR::load(cigar, position).expect("CIGAR loading failed!"),
  ///   chr:      chr.clone(),
  ///   flag:     flag,
  ///   position: position,
  ///   mapq:     mapq,
  ///   tlen:     tlen,
  /// };
  ///
  /// assert_eq!(produced, manual);
  /// ```
  pub fn update(
    &mut self,
    cigar: CIGAR,
    chr: String,
    flag: i32,
    mapq: i32,
    position: i32,
    tlen: i32,
  ) {
    // TODO: ignore anchor for now self.anchor = AnchorEnum::None;
    self.cigar = cigar;
    self.chr = chr;
    self.flag = flag;
    self.mapq = mapq;
    self.position = position;
    self.tlen = tlen;
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// bin
impl Anchor for ChrAnchor {
  fn get_position(&self) -> i32 {
    self.position
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// implement SAM flag
impl SAMFlag for ChrAnchor {
  fn get_flag(&self) -> i32 {
    self.flag
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// display trait implementation
impl fmt::Display for ChrAnchor {
  fn fmt(
    &self,
    f: &mut fmt::Formatter,
  ) -> fmt::Result {
    writeln!(
      f,
      "{}\t{}\t{}\t{}\t",
      self.chr, self.position, self.cigar, self.tlen
    )
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
