////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::structures::cigar::CIGAR;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, new, Clone, Default)]
pub struct RawValues {
  #[new(value = "CIGAR::new()")]
  pub cigar: CIGAR,

  #[new(default)]
  pub scaffold: String,

  #[new(default)]
  pub flag: i32,

  #[new(default)]
  pub position: i32,

  #[new(default)]
  pub quality: i32,

  #[new(default)]
  pub read_id: String,

  #[new(default)]
  pub sequence: String,

  #[new(default)]
  pub tlen: i32,
  // TODO: expand to other annotations?
}

////////////////////////////////////////////////////////////////////////////////////////////////////
