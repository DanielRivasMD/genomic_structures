////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
// use std::borrow::Borrow;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::custom::{
  chr_anchor_enum::ChrAnchorEnum,
  me_anchor::MEAnchor,
  me_chimeric_read::MEChimericRead,
  orientation_enum::OrientationEnum,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// to load onto => hashmap for reads aligned to mobile elements
/// Contain information about a chimeric pair.
#[derive(Debug, new, Default, PartialEq)]
pub struct MEChimericPair {
  /// Chimeric read 1.
  #[new(default)]
  pub read1: MEChimericRead,

  /// Chimeric read 2.
  #[new(default)]
  pub read2: MEChimericRead,

  /// Chromosomal anchor identifier.
  #[new(default)]
  pub chranch: ChrAnchorEnum,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// load & update
impl MEChimericPair {
  ///
  pub fn load(me_anchor: MEAnchor) -> Self {
    let mut me_chimeric_pair = Self::new();
    me_chimeric_pair.read1 = MEChimericRead::load(me_anchor);
    return me_chimeric_pair;
  }

  ///
  pub fn update(&mut self) {
    unimplemented!();
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// get & tag chromosomal anchor enum
impl MEChimericPair {
  ///
  /// Retrieve chromosomal anchor.
  ///
  /// # Examples
  ///
  /// ```
  /// use genomic_structures::{
  ///   ChrAnchorEnum,
  ///   MEChimericPair,
  ///   MEChimericRead,
  /// };
  ///
  /// let mut to_retrieve = MEChimericPair::new();
  /// to_retrieve.read1.sequence = "GATTACA".to_string();
  /// let retrieved = to_retrieve.get_chr_anchor();
  ///
  /// let mut predefined = MEChimericRead::new();
  /// predefined.sequence = "GATTACA".to_string();
  ///
  /// assert_eq!(retrieved.sequence, predefined.sequence);
  /// ```
  pub fn get_chr_anchor(&self) -> &MEChimericRead {
    match self.chranch {
      ChrAnchorEnum::Read1 => &self.read1,
      ChrAnchorEnum::Read2 => &self.read2,
      ChrAnchorEnum::None => {
        // TODO: think about an alternative here
        println!("This is a default value");
        &self.read1
      }
    }
    // .borrow()
  }

  // if both reads in the pair align to LTR at the same position that would
  // imply them being super reads & overlapping in alignment. therefore, it
  // becomes ambigous to anchor.
  // if reads are palindromic & align to both ends of mobile element it becomes
  // ambigous to anchor
  ///
  pub fn tag(&mut self) {
    // tag each read
    self.read1.tag();
    self.read2.tag();
    let edges = (self.read1.edge(), self.read2.edge());

    self.chranch = match (self.read1.orientation, self.read2.orientation) {
      // upstream
      (OrientationEnum::Upstream, OrientationEnum::Upstream) => {
        match edges {
          (r1, r2) if r1 < r2 => ChrAnchorEnum::Read1,
          (r1, r2) if r1 > r2 => ChrAnchorEnum::Read2,
          (_, _) => ChrAnchorEnum::None,
        }
      }
      (OrientationEnum::Upstream, OrientationEnum::None) => {
        ChrAnchorEnum::Read2
      }
      (OrientationEnum::Upstream, _) => ChrAnchorEnum::None,

      // downstream
      (OrientationEnum::Downstream, OrientationEnum::Downstream) => {
        match edges {
          (r1, r2) if r1 < r2 => ChrAnchorEnum::Read2,
          (r1, r2) if r1 > r2 => ChrAnchorEnum::Read1,
          (_, _) => ChrAnchorEnum::None,
        }
      }
      (OrientationEnum::Downstream, OrientationEnum::None) => {
        ChrAnchorEnum::Read2
      }
      (OrientationEnum::Downstream, _) => ChrAnchorEnum::None,

      // palindromic
      (OrientationEnum::Palindromic, _) => ChrAnchorEnum::None,

      // none
      (OrientationEnum::None, OrientationEnum::Upstream) => {
        ChrAnchorEnum::Read1
      }
      (OrientationEnum::None, OrientationEnum::Downstream) => {
        ChrAnchorEnum::Read1
      }
      (OrientationEnum::None, _) => ChrAnchorEnum::None,
    }
  }
  // TODO: add trait implementation for mobile element retrieval
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// display trait implementation
impl fmt::Display for MEChimericPair {
  fn fmt(
    &self,
    f: &mut fmt::Formatter,
  ) -> fmt::Result {
    writeln!(f, "{}\n{}\n", self.read1, self.read2)
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
