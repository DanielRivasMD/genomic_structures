////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::fmt;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::structures::{
  chr_anchor::ChrAnchor,
  me_anchor::MEAnchor,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Contain primary (index 0) and secondary aligned read annotation.
#[derive(Debug, new, Default, PartialEq)]
pub struct MEChimericRead {
  /// Chromosomal anchor.
  #[new(default)]
  pub chr_read: Vec<ChrAnchor>,

  /// Mobile element anchor.
  #[new(default)]
  pub me_read: Vec<MEAnchor>,

  /// Anchor mapping quality (MAPQ).
  #[new(default)]
  pub quality: i32,

  /// Sequence.
  #[new(default)]
  pub sequence: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// load & update
impl MEChimericRead {
  pub fn load(me_anchor: MEAnchor) -> Self {
    let mut me_chimeric_read = Self::new();
    me_chimeric_read.me_read.push(me_anchor);
    return me_chimeric_read;
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// reverse sequence
impl MEChimericRead {
  ///
  /// Obtain reverse complement sequence.
  ///
  /// # Examples
  ///
  /// ```
  /// use genomic_structures::MEChimericRead;
  ///
  /// let mut to_reverse = MEChimericRead::new();
  /// to_reverse.sequence = "GATTACA".to_string();
  /// let reversed = "TGTAATC".to_string();
  ///
  /// assert_eq!(to_reverse.reverse_sequence(), reversed);
  /// ```
  pub fn reverse_sequence(&self) -> String {
    self
      .sequence
      .chars()
      .map(|nucleotide| {
        match nucleotide {
          '!' => '?',
          'A' => 'T',
          'T' => 'A',
          'C' => 'G',
          'G' => 'C',
          _ => nucleotide,
        }
      })
      .rev()
      .collect()
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// manual display trait implementation
impl fmt::Display for MEChimericRead {
  fn fmt(
    &self,
    f: &mut fmt::Formatter,
  ) -> fmt::Result {
    writeln!(
      f,
      "Chromosome: {}, Position: {}",
      self.chr_read[0].chr, self.chr_read[0].position,
    )
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
