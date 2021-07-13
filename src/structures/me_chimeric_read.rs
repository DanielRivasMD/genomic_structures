////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::fmt;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::structures::{
  break_point::BreakPoint,
  chr_anchor::ChrAnchor,
  me_anchor::MEAnchor,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Contain primary (index 0) and secondary aligned reads annotation.
#[derive(Debug, new, Default)]
pub struct MEChimericRead {
  /// Sequence.
  #[new(default)]
  pub sequence: String,

  /// Mobile element read.
  #[new(default)]
  pub me_read: Vec<MEAnchor>,

  /// Chromosomal read.
  #[new(default)]
  pub chr_read: Vec<ChrAnchor>,

  /// Breakpoint.
  #[new(default)]
  pub breakpoint: BreakPoint,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl MEChimericRead {
  /// Obtain reverse complement sequence.
  ///
  /// # Examples
  ///
  /// ```
  /// use genomic_structures::MEChimericRead;
  ///
  /// let mut toreverse = MEChimericRead::new();
  /// toreverse.sequence = "GATTACA".to_string();
  /// let reversed = "TGTAATC".to_string();
  ///
  /// assert_eq!(toreverse.sequence_reverser(), reversed);
  /// ```
  pub fn sequence_reverser(&self) -> String {
    self
      .sequence
      .chars()
      .map(|x| {
        match x {
          '!' => '?',
          'A' => 'T',
          'T' => 'A',
          'C' => 'G',
          'G' => 'C',
          _ => x,
        }
      })
      .rev()
      .collect()
  }

  //  TODO: add breakpoint determination as trait
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// display trait implementation
impl fmt::Display for MEChimericRead {
  fn fmt(
    &self,
    f: &mut fmt::Formatter,
  ) -> fmt::Result {
    writeln!(
      f,
      "Chromosome: {}, Position: {}",
      self.chr_read[0].chr, self.chr_read[0].pos,
    )
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
