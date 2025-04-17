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
  ///
  pub fn load(me_anchor: MEAnchor) -> Self {
    let mut me_chimeric_read = Self::new();
    me_chimeric_read.me_read.push(me_anchor);
    return me_chimeric_read;
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// tag
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
      OrientationPair(u, d) if u == d && u != 0 => OrientationEnum::Palindromic,
      OrientationPair(_, _) => OrientationEnum::None,
    }
  }

  ///
  pub fn edge(&self) -> i32 {
    // TODO: better way to work with options?
    match self.orientation {
      OrientationEnum::Upstream => {
        let mut boundries = Vec::new();
        self.me_read.iter().for_each(|me_anchor| {
          if me_anchor.orientation == self.orientation {
            boundries.push(me_anchor.cigar.left_boundry);
          }
        });
        match boundries.into_iter().min() {
          Some(min) => min,
          None => 0,
        }
      }

      OrientationEnum::Downstream => {
        let mut boundries = Vec::new();
        self.me_read.iter().for_each(|me_anchor| {
          if me_anchor.orientation == self.orientation {
            boundries.push(me_anchor.cigar.right_boundry);
          }
        });
        match boundries.into_iter().max() {
          Some(max) => max,
          None => 0,
        }
      }

      OrientationEnum::Palindromic => 0,

      OrientationEnum::None => 0,
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

// display trait implementation
impl fmt::Display for MEChimericRead {
  fn fmt(
    &self,
    f: &mut fmt::Formatter,
  ) -> fmt::Result {
    writeln!(
      f,
      "{}\t{}\t{}\t{}\t",
      self.chr_read[0], self.me_read[0], self.quality, self.sequence,
    )
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
