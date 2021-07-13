////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::structures::{
  break_point::BreakPoint,
  chr_anchor::ChrAnchor,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Contain primary (index 0) and secondary aligned read annotations.
#[derive(Debug, new, Default)]
pub struct SVChimericRead {
  /// Sequence.
  #[new(default)]
  pub sequence: String,

  /// Chromosomal read.
  #[new(default)]
  pub chr_read: Vec<ChrAnchor>,

  /// Breakpoint.
  #[new(default)]
  pub breakpoint: BreakPoint,
}

////////////////////////////////////////////////////////////////////////////////////////////////////