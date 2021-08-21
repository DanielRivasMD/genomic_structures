////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::fmt;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  functions::sequences::Sequence,
  structures::{
    chr_anchor::ChrAnchor,
    me_anchor::MEAnchor,
    orientation_enum::OrientationEnum,
  },
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

  #[new(default)]
  pub orientation: OrientationEnum,

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

// TODO: finish tagging implementation
impl MEChimericRead {
  ///
  pub fn tag(&mut self) {
    let upstream = anchor_count!(self, Upstream);
    let downstream = anchor_count!(self, Downstream);

    if upstream > downstream {
      self.orientation = OrientationEnum::Upstream;
    } else if upstream < downstream {
      self.orientation = OrientationEnum::Downstream;
    } else {
      self.orientation = OrientationEnum::Palindromic;
    }
  }

// reverse sequence
impl Sequence for MEChimericRead {
  fn get_sequence(&self) -> &str {
    &self.sequence
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
