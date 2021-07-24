////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  functions::identificator::identify,
  structures::{
    sv_chimeric_read::SVChimericRead,
    sv_type::SVType,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// to load onto => hashmap for reads aligned to mobile elements
/// Contain information about a chimeric pair.
#[derive(Debug, new)]
pub struct SVChimericPair {
  /// Read 1.
  #[new(default)]
  pub read1: SVChimericRead,

  /// Read 2.
  #[new(default)]
  pub read2: SVChimericRead,

  /// Strucutural variant type.
  pub svtag: SVType,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl SVChimericPair {
  /// Identify type of structural variant.
  pub fn identify(
    &mut self,
    expected_tlen: i32,
  ) -> bool {
    identify(self, expected_tlen)
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
