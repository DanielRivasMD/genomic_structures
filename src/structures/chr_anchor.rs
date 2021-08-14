////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::BIN_SIZE;
use crate::{
  functions::sam_flag::SAMFlag,
  structures::{
    anchor_enum::AnchorEnum,
    cigar::CIGAR,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Chromosomal anchor structure.
#[derive(Debug, new, Clone, Default, PartialEq)]
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

// create
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
  /// let chr = String::from("chr7");
  /// let flag = 56;
  /// let position = 2099;
  /// let mapq = 60;
  /// let tlen = 100;
  ///
  /// let produced = ChrAnchor::load(
  ///   CIGAR::load(cigar, position).unwrap(),
  ///   chr.clone(),
  ///   flag,
  ///   mapq,
  ///   position,
  ///   tlen,
  /// );
  ///
  /// let manual = ChrAnchor {
  ///   anchor:   AnchorEnum::None,
  ///   cigar:    CIGAR::load(cigar, position).unwrap(),
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
  /// let chr = String::from("chr7");
  /// let flag = 56;
  /// let position = 2099;
  /// let mapq = 60;
  /// let tlen = 100;
  ///
  /// let mut produced = ChrAnchor::new();
  /// produced.update(
  ///   CIGAR::load(cigar, position).unwrap(),
  ///   chr.clone(),
  ///   flag,
  ///   mapq,
  ///   position,
  ///   tlen,
  /// );
  ///
  /// let manual = ChrAnchor {
  ///   anchor:   AnchorEnum::None,
  ///   cigar:    CIGAR::load(cigar, position).unwrap(),
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
    // TODO: update initializers
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

// methods
impl ChrAnchor {
  ///
  /// Bin alignment position.
  ///
  /// # Returns
  /// Return binned position.
  ///
  /// # Examples
  ///
  /// ```
  /// use genomic_structures::{
  ///   ChrAnchor,
  ///   CIGAR,
  /// };
  ///
  /// let cigar = "100M";
  /// let chr = String::from("chr7");
  /// let flag = 56;
  /// let position = 2099;
  /// let mapq = 60;
  /// let tlen = 100;
  ///
  /// let chr_anchor = ChrAnchor::load(
  ///   CIGAR::load(cigar, position).unwrap(),
  ///   chr,
  ///   flag,
  ///   mapq,
  ///   position,
  ///   tlen,
  /// );
  ///
  /// assert_eq!(chr_anchor.bin(), 2000);
  /// ```
  pub fn bin(&self) -> i32 {
    let binned = self.position % BIN_SIZE;
    self.position - binned
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
