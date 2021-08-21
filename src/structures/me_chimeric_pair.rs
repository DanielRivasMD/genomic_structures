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
    .borrow()
  }

  pub fn tag(&mut self) {
    self.read1.tag();
    self.read2.tag();

    let tags = (self.read1.orientation, self.read2.orientation);

    match tags {
      (OrientationEnum::Upstream, OrientationEnum::Upstream) => {}
      (OrientationEnum::Upstream, OrientationEnum::Downstream) => {}
      (OrientationEnum::Upstream, OrientationEnum::Palindromic) => {}
      (OrientationEnum::Upstream, OrientationEnum::None) => {}

      (OrientationEnum::Downstream, OrientationEnum::Upstream) => {}
      (OrientationEnum::Downstream, OrientationEnum::Downstream) => {}
      (OrientationEnum::Downstream, OrientationEnum::Palindromic) => {}
      (OrientationEnum::Downstream, OrientationEnum::None) => {}

      (OrientationEnum::Palindromic, OrientationEnum::Upstream) => {}
      (OrientationEnum::Palindromic, OrientationEnum::Downstream) => {}
      (OrientationEnum::Palindromic, OrientationEnum::Palindromic) => {}
      (OrientationEnum::Palindromic, OrientationEnum::None) => {}

      (OrientationEnum::None, OrientationEnum::Upstream) => {}
      (OrientationEnum::None, OrientationEnum::Downstream) => {}
      (OrientationEnum::None, OrientationEnum::Palindromic) => {}
      (OrientationEnum::None, OrientationEnum::None) => {}
    }

    if self.read1.orientation == OrientationEnum::Upstream &&
      self.read2.orientation == OrientationEnum::Upstream
    {
      let read1 = self.read1.count_tag().unwrap();
      let read2 = self.read2.count_tag().unwrap();
      if read1 < read2 {
        self.chranch = ChrAnchorEnum::Read1;
      } else if read1 > read2 {
        self.chranch = ChrAnchorEnum::Read2;
      } else {
        println!("Probably ambigous");
      }
    } else if self.read1.orientation == OrientationEnum::Downstream &&
      self.read2.orientation == OrientationEnum::Downstream
    {
      let read1 = self.read1.count_tag().unwrap();
      let read2 = self.read2.count_tag().unwrap();
      if read1 < read2 {
        self.chranch = ChrAnchorEnum::Read2;
      } else if read1 > read2 {
        self.chranch = ChrAnchorEnum::Read1;
      } else {
        println!("Probably ambigous");
      }
    } else if self.read1.orientation == OrientationEnum::Palindromic &&
      self.read2.orientation == OrientationEnum::Palindromic
    {
      println!("Palindromic");
    }
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
