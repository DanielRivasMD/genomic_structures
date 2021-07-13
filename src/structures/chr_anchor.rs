////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::functions::flag_interpretor::{
  interpretor,
  SamFlag,
};
use crate::BIN_SIZE;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Chromosomal anchor structure.
#[derive(Debug, new, Default)]
pub struct ChrAnchor {
  /// Chromosome.
  #[new(default)]
  pub chr: String,

  /// Flag.
  #[new(default)]
  pub flag: i32,

  /// Position.
  #[new(default)]
  pub pos: i32,

  /// CIGAR.
  #[new(default)]
  pub cigar: String,

  /// Mapping quality (MAPQ).
  #[new(default)]
  pub mapq: i32,

  /// Template length (TLEN).
  #[new(default)]
  pub tlen: i32,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl ChrAnchor {
  /// Load vector of strings (line from a file) onto ChrAnchor struct.
  ///
  /// # Examples
  ///
  /// ```
  /// use genomic_structures::ChrAnchor;
  ///
  /// let loaded = ChrAnchor::loader(&vec![
  ///   "", "56", "chr7", "2099", "100", "100M", "", "", "100",
  /// ]);
  /// let manual = ChrAnchor {
  ///   chr:   "chr7".to_string(),
  ///   flag:  56,
  ///   pos:   2099,
  ///   cigar: "100M".to_string(),
  ///   mapq:  100,
  ///   tlen:  100,
  /// };
  ///
  /// assert_eq!(loaded.chr, manual.chr);
  /// assert_eq!(loaded.flag, manual.flag);
  /// assert_eq!(loaded.pos, manual.pos);
  /// assert_eq!(loaded.cigar, manual.cigar);
  /// assert_eq!(loaded.mapq, manual.mapq);
  /// assert_eq!(loaded.tlen, manual.tlen);
  /// ```
  pub fn loader(file_line: &[&str]) -> Self {
    Self {
      chr:   file_line[2].to_string(),
      flag:  file_line[1].parse::<i32>().unwrap(),
      pos:   file_line[3].parse::<i32>().unwrap(),
      cigar: file_line[5].to_string(),
      mapq:  file_line[4].parse::<i32>().unwrap(),
      tlen:  file_line[8].parse::<i32>().unwrap(),
    }
  }

  /// Bin chromosomal position.
  ///
  /// # Examples
  ///
  /// ```
  /// use genomic_structures::ChrAnchor;
  ///
  /// assert_eq!(
  ///   ChrAnchor {
  ///     chr:   "chr7".to_string(),
  ///     flag:  56,
  ///     pos:   2099,
  ///     cigar: "100M".to_string(),
  ///     mapq:  100,
  ///     tlen:  100,
  ///   }
  ///   .binner(),
  ///   2000
  /// );
  /// ```
  pub fn binner(&self) -> i32 {
    let binned = self.pos % BIN_SIZE;
    self.pos - binned
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl SamFlag for ChrAnchor {
  fn interpretor(
    &self,
    p: usize,
  ) -> bool {
    interpretor(self.flag, p)
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
