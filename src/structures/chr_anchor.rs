////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::BIN_SIZE;
use crate::{
  functions::{
    flag_interpretor::interpret,
    sam_flag::SamFlag,
  },
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

  /// CIGAR.
  #[new(value = "CIGAR::new()")]
  pub cigar: CIGAR,

  /// Flag.
  #[new(default)]
  pub flag: i32,

  /// Chromosome.
  #[new(default)]
  pub chr: String,

  /// Mapping quality (MAPQ).
  #[new(default)]
  pub mapq: i32,

  /// Position.
  #[new(default)]
  pub position: i32,

  /// Template length (TLEN).
  #[new(default)]
  pub tlen: i32,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl ChrAnchor {
//  /// use genomic_structures::ChrAnchor;
//  ///
//  /// let loaded = ChrAnchor::loader(&vec![
//  ///   "", "56", "chr7", "2099", "100", "100M", "", "", "100",
//  /// ]);
//  /// let manual = ChrAnchor {
//  ///   chr:   "chr7".to_string(),
//  ///   flag:  56,
//  ///   pos:   2099,
//  ///   cigar: "100M".to_string(),
//  ///   mapq:  100,
//  ///   tlen:  100,
//  /// };
//  ///
//  /// assert_eq!(loaded.chr, manual.chr);
//  /// assert_eq!(loaded.flag, manual.flag);
//  /// assert_eq!(loaded.pos, manual.pos);
//  /// assert_eq!(loaded.cigar, manual.cigar);
//  /// assert_eq!(loaded.mapq, manual.mapq);
//  /// assert_eq!(loaded.tlen, manual.tlen);
  /// Load vector of strings (line from a file) onto ChrAnchor struct.
  ///
  /// # Examples
  ///
  /// ```
  /// ```
  pub fn load(
    cigar: CIGAR,
    flag: i32,
    chr: String,
    mapq: i32,
    position: i32,
    tlen: i32,
  ) -> Self {
    Self {
      anchor: AnchorEnum::None,
      cigar,
      flag,
      chr,
      mapq,
      position,
      tlen,
    }
  }

  pub fn update(
    &mut self,
    cigar: CIGAR,
    flag: i32,
    chr: String,
    mapq: i32,
    position: i32,
    tlen: i32,
  ) {
    // TODO: update initializers
    self.anchor = AnchorEnum::None;
    self.cigar = cigar;
    self.flag = flag;
    self.chr = chr;
    self.mapq = mapq;
    self.position = position;
    self.tlen = tlen;
  }

  //  /// use genomic_structures::ChrAnchor;
  //  ///
  //  /// assert_eq!(
  //  ///   ChrAnchor {
  //  ///     chr:   "chr7".to_string(),
  //  ///     flag:  56,
  //  ///     pos:   2099,
  //  ///     cigar: "100M".to_string(),
  //  ///     mapq:  100,
  //  ///     tlen:  100,
  //  ///   }
  //  ///   .binner(),
  //  ///   2000
  //  /// );
  /// Bin chromosomal position.
  ///
  /// # Examples
  ///
  /// ```
  /// ```
  pub fn bin(&self) -> i32 {
    let binned = self.position % BIN_SIZE;
    self.position - binned
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl SamFlag for ChrAnchor {
  /// Binary interpretation on a SAM flag for ChrAnchor struct.
  fn interpret(
    &self,
    p: usize,
  ) -> bool {
    interpret(self.flag, p)
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
