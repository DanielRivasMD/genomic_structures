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
////////////////////////////////////////////////////////////////////////////////////////////////////

impl RawValues {
  pub fn mobel_tag(
    &self,
    // me_anchor: MEAnchor,
    me_limit: i32,
    orientation: bool
  ) -> String {
    if self.cigar.left_boundry <= me_limit && orientation {
      return String::from("upstream");
    } else if self.cigar.right_boundry as f64 <= me_limit.into() && !orientation {
      return String::from("downstream");
    } else {
      return String::new();
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
