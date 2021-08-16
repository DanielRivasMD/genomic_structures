////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::borrow::Borrow;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::structures::{
  chr_anchor_enum::ChrAnchorEnum,
  me_anchor::MEAnchor,
  me_chimeric_read::MEChimericRead,
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
  pub fn load(me_anchor: MEAnchor) -> Self {
    let mut me_chimeric_pair = Self::new();
    me_chimeric_pair.read1 = MEChimericRead::load(me_anchor);
    return me_chimeric_pair;
  }

  pub fn update(&mut self) {
    unimplemented!();
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// get
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
    .borrow()
  }

  // TODO: add trait implementation for mobile element retrieval
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// manual display trait implementation
impl fmt::Display for MEChimericPair {
  fn fmt(
    &self,
    f: &mut fmt::Formatter,
  ) -> fmt::Result {
    writeln!(f, "Read 1: {}\n=====\nRead 2: {}", self.read1, self.read2,)
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
