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
    orientation_enum::{
      OrientationEnum,
      OrientationPair,
    },
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
    let orientation = OrientationPair(
      anchor_count!(self, Upstream),
      anchor_count!(self, Downstream),
    );

    self.orientation = match orientation {
      OrientationPair(u, d) if u > d => OrientationEnum::Upstream,
      OrientationPair(u, d) if u < d => OrientationEnum::Downstream,
      OrientationPair(u, d) if u == d => OrientationEnum::Palindromic,
      // TODO: check that this expresion does not bug single tags
      OrientationPair(_, _) => OrientationEnum::None,
    }
  }

  pub fn count_tag(&self) -> Option<i32> {
    let mut positions = Vec::new();
    self.me_read.iter().for_each(|me_anchor| {
      if me_anchor.orientation == self.orientation {
        positions.push(me_anchor.position);
      }
    });

    match self.orientation {
      OrientationEnum::Upstream => positions.into_iter().min(),
      OrientationEnum::Downstream => positions.into_iter().max(),
      OrientationEnum::Palindromic => None,
      OrientationEnum::None => None,
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

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
